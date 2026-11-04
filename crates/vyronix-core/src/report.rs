//! Report model and offline serialization (JSON / HTML / CSV).
//!
//! Reports are professional, branded, print-friendly and contain only real
//! collected data with honest status labels. HTML output escapes all dynamic
//! values to prevent injection. PDF is produced by the frontend from this
//! same model (offline, via jsPDF).

use serde::{Deserialize, Serialize};

use crate::diagnostics::Finding;
use crate::recommend::Recommendation;
use crate::scoring::OverallScore;

/// A single labelled value in a report, with provenance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportEntry {
    pub label: String,
    pub value: String,
    pub status: String,
    pub source: String,
}

/// A titled group of entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportSection {
    pub title: String,
    pub entries: Vec<ReportEntry>,
}

/// Device identity summary (only non-sensitive fields by default).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceSummary {
    pub device_name: String,
    pub manufacturer: String,
    pub model: String,
    pub os: String,
}

/// The full report model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportModel {
    pub title: String,
    pub report_type: String,
    pub generated_at_ms: u64,
    pub app_version: String,
    pub device: DeviceSummary,
    pub sections: Vec<ReportSection>,
    pub findings: Vec<Finding>,
    pub overall: Option<OverallScore>,
    pub recommendations: Vec<Recommendation>,
    /// Whether sensitive identifiers (serials, MAC, IP) were included by
    /// explicit user consent.
    pub include_sensitive: bool,
}

impl ReportModel {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Escape a string for safe inclusion in HTML text/attributes.
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape a CSV field per RFC 4180.
pub fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn severity_color(sev: &str) -> &'static str {
    match sev {
        "good" => "#16a34a",
        "information" => "#0284c7",
        "warning" => "#d97706",
        "critical" => "#dc2626",
        _ => "#6b7280",
    }
}

/// Render a professional, print-friendly HTML report. All dynamic content is
/// escaped. Works fully offline (inline CSS, no external resources).
pub fn to_html(model: &ReportModel) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
    html.push_str(&format!("<title>{}</title>", escape_html(&model.title)));
    html.push_str("<style>");
    html.push_str("*{box-sizing:border-box}body{font-family:'Segoe UI',system-ui,-apple-system,sans-serif;margin:0;color:#111827;background:#f9fafb;line-height:1.5}");
    html.push_str(".wrap{max-width:900px;margin:0 auto;padding:32px 24px}");
    html.push_str("header{border-bottom:3px solid #0ea5e9;padding-bottom:16px;margin-bottom:24px}");
    html.push_str(".brand{font-size:13px;letter-spacing:.2em;color:#0ea5e9;font-weight:700;text-transform:uppercase}");
    html.push_str("h1{font-size:26px;margin:4px 0}h2{font-size:18px;margin:28px 0 10px;border-left:4px solid #0ea5e9;padding-left:10px}");
    html.push_str(".meta{color:#6b7280;font-size:13px}");
    html.push_str("table{width:100%;border-collapse:collapse;margin:8px 0;font-size:14px}");
    html.push_str("th,td{text-align:left;padding:8px 10px;border-bottom:1px solid #e5e7eb;vertical-align:top}");
    html.push_str("th{background:#f3f4f6;font-weight:600}");
    html.push_str(".badge{display:inline-block;padding:2px 8px;border-radius:999px;font-size:12px;font-weight:600;color:#fff}");
    html.push_str(".src{color:#9ca3af;font-size:12px}");
    html.push_str(".finding{border:1px solid #e5e7eb;border-radius:8px;padding:12px 14px;margin:8px 0;background:#fff}");
    html.push_str(".finding h3{margin:0 0 6px;font-size:15px}.finding p{margin:4px 0;font-size:13px}");
    html.push_str(".score{font-size:40px;font-weight:700}.muted{color:#6b7280}");
    html.push_str("footer{margin-top:32px;border-top:1px solid #e5e7eb;padding-top:12px;font-size:12px;color:#9ca3af}");
    html.push_str("@media print{body{background:#fff}.wrap{max-width:100%}}");
    html.push_str("</style></head><body><div class=\"wrap\">");

    html.push_str("<header>");
    html.push_str("<div class=\"brand\">MERQO &middot; VYRONIX</div>");
    html.push_str(&format!("<h1>{}</h1>", escape_html(&model.title)));
    html.push_str(&format!(
        "<div class=\"meta\">Generated {} &middot; VYRONIX v{} &middot; {}</div>",
        escape_html(&format_timestamp(model.generated_at_ms)),
        escape_html(&model.app_version),
        escape_html(&model.device.device_name)
    ));
    html.push_str("</header>");

    // Device summary.
    html.push_str("<h2>Device</h2><table>");
    for (k, v) in [
        ("Device name", &model.device.device_name),
        ("Manufacturer", &model.device.manufacturer),
        ("Model", &model.device.model),
        ("Operating system", &model.device.os),
    ] {
        html.push_str(&format!("<tr><th>{}</th><td>{}</td></tr>", escape_html(k), escape_html(v)));
    }
    html.push_str("</table>");

    // Overall score.
    if let Some(overall) = &model.overall {
        html.push_str("<h2>Overall device condition</h2>");
        match overall.score {
            Some(s) => {
                html.push_str(&format!(
                    "<div class=\"score\">{:.0}<span class=\"muted\" style=\"font-size:16px\"> / 100</span></div>",
                    s
                ));
                html.push_str(&format!(
                    "<div class=\"muted\">Coverage {:.0}% &middot; Confidence: {} &middot; {}/{} components assessed. Unsupported components are excluded, not awarded points.</div>",
                    overall.coverage * 100.0,
                    escape_html(overall.confidence.label()),
                    overall.included_count, overall.total_count
                ));
            }
            None => html.push_str("<div class=\"muted\">Insufficient measurable data for an overall score.</div>"),
        }
        html.push_str("<table><tr><th>Component</th><th>Score</th><th>Status</th><th>Rationale</th></tr>");
        for c in &overall.components {
            let score_txt = c.score.map(|s| format!("{s:.0}")).unwrap_or_else(|| "—".into());
            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                escape_html(&c.label), escape_html(&score_txt), escape_html(c.severity.label()), escape_html(&c.rationale)
            ));
        }
        html.push_str("</table>");
    }

    // Sections.
    for section in &model.sections {
        html.push_str(&format!("<h2>{}</h2>", escape_html(&section.title)));
        html.push_str("<table><tr><th>Item</th><th>Value</th><th>Status</th><th>Source</th></tr>");
        for e in &section.entries {
            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"src\">{}</td></tr>",
                escape_html(&e.label), escape_html(&e.value), escape_html(&e.status), escape_html(&e.source)
            ));
        }
        html.push_str("</table>");
    }

    // Findings.
    if !model.findings.is_empty() {
        html.push_str("<h2>Diagnostic findings</h2>");
        for f in &model.findings {
            let color = severity_color(&serde_lower(f.severity.label()));
            html.push_str("<div class=\"finding\">");
            html.push_str(&format!(
                "<h3>{} <span class=\"badge\" style=\"background:{}\">{}</span></h3>",
                escape_html(&f.finding), color, escape_html(f.severity.label())
            ));
            html.push_str(&format!("<p><strong>Evidence:</strong> {}</p>", escape_html(&f.evidence)));
            html.push_str(&format!("<p><strong>Explanation:</strong> {}</p>", escape_html(&f.explanation)));
            html.push_str(&format!("<p><strong>Recommendation:</strong> {}</p>", escape_html(&f.recommendation)));
            html.push_str(&format!("<p class=\"src\">Source: {} &middot; Confidence: {:?}</p>", escape_html(&f.data_source), f.confidence));
            html.push_str("</div>");
        }
    }

    // Recommendations.
    if !model.recommendations.is_empty() {
        html.push_str("<h2>Usage recommendations</h2><table><tr><th>Use case</th><th>Suitability</th><th>Rationale</th></tr>");
        for r in &model.recommendations {
            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                escape_html(&r.use_case), escape_html(r.suitability.label()), escape_html(&r.rationale)
            ));
        }
        html.push_str("</table>");
    }

    html.push_str("<footer>");
    html.push_str("VYRONIX &middot; A MERQO Software Product &middot; Founder: Shohan Khan &middot; merqoonline@gmail.com<br>");
    html.push_str("Know Your Hardware. Trust Your Device. &middot; This report contains only locally collected data. ");
    if !model.include_sensitive {
        html.push_str("Sensitive identifiers (serial numbers, MAC/IP addresses) were excluded.");
    }
    html.push_str("</footer></div></body></html>");
    html
}

fn serde_lower(s: &str) -> String {
    s.to_ascii_lowercase()
}

/// Render the report as CSV (sections + findings flattened).
pub fn to_csv(model: &ReportModel) -> String {
    let mut out = String::new();
    out.push_str("section,item,value,status,source\n");
    for section in &model.sections {
        for e in &section.entries {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                escape_csv(&section.title), escape_csv(&e.label), escape_csv(&e.value),
                escape_csv(&e.status), escape_csv(&e.source)
            ));
        }
    }
    if !model.findings.is_empty() {
        out.push_str("\ncategory,finding,severity,evidence,recommendation\n");
        for f in &model.findings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                escape_csv(&f.category), escape_csv(&f.finding), escape_csv(f.severity.label()),
                escape_csv(&f.evidence), escape_csv(&f.recommendation)
            ));
        }
    }
    out
}

/// Format a Unix-millis timestamp as a UTC ISO-like string without external deps.
pub fn format_timestamp(ms: u64) -> String {
    let secs = ms / 1000;
    let days = secs / 86_400;
    let rem = secs % 86_400;
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    // Civil-from-days algorithm (Howard Hinnant) to get Y-M-D.
    let z = days as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mth = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if mth <= 2 { y + 1 } else { y };
    format!("{year:04}-{mth:02}-{d:02} {h:02}:{m:02}:{s:02} UTC")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Confidence, Severity};

    fn sample() -> ReportModel {
        ReportModel {
            title: "Full System Report".into(),
            report_type: "full".into(),
            generated_at_ms: 1_700_000_000_000,
            app_version: "0.1.0".into(),
            device: DeviceSummary {
                device_name: "TEST-PC".into(),
                manufacturer: "TestMaker".into(),
                model: "Model X".into(),
                os: "Windows 11".into(),
            },
            sections: vec![ReportSection {
                title: "Battery".into(),
                entries: vec![ReportEntry {
                    label: "Health".into(), value: "90%".into(),
                    status: "Available".into(), source: "IOCTL".into(),
                }],
            }],
            findings: vec![Finding::new(
                "battery_health", "Battery", "Battery health is good", Severity::Good,
                "90%", "IOCTL", "explanation", "recommendation", Confidence::High,
            )],
            overall: None,
            recommendations: vec![],
            include_sensitive: false,
        }
    }

    #[test]
    fn json_roundtrip() {
        let m = sample();
        let json = m.to_json().unwrap();
        let back: ReportModel = serde_json::from_str(&json).unwrap();
        assert_eq!(m, back);
    }

    #[test]
    fn html_escapes_injection() {
        let mut m = sample();
        m.device.device_name = "<script>alert('x')</script>".into();
        let html = to_html(&m);
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>alert"));
    }

    #[test]
    fn html_contains_branding_and_sections() {
        let html = to_html(&sample());
        assert!(html.contains("MERQO"));
        assert!(html.contains("VYRONIX"));
        assert!(html.contains("Battery"));
        assert!(html.contains("90%"));
        assert!(html.contains("Shohan Khan"));
    }

    #[test]
    fn csv_escapes_commas_and_quotes() {
        assert_eq!(escape_csv("plain"), "plain");
        assert_eq!(escape_csv("a,b"), "\"a,b\"");
        assert_eq!(escape_csv("a\"b"), "\"a\"\"b\"");
    }

    #[test]
    fn csv_output_has_header_and_rows() {
        let csv = to_csv(&sample());
        assert!(csv.starts_with("section,item,value,status,source"));
        assert!(csv.contains("Battery,Health,90%"));
    }

    #[test]
    fn timestamp_format() {
        // 1_700_000_000_000 ms = 2023-11-14 22:13:20 UTC
        assert_eq!(format_timestamp(1_700_000_000_000), "2023-11-14 22:13:20 UTC");
        assert_eq!(format_timestamp(0), "1970-01-01 00:00:00 UTC");
    }

    #[test]
    fn sensitive_note_present_when_excluded() {
        let html = to_html(&sample());
        assert!(html.contains("Sensitive identifiers"));
    }
}
