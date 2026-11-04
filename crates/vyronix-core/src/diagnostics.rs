//! Transparent rule-based diagnostic engine.
//!
//! Every finding records its evidence, data source, explanation,
//! recommendation, confidence and timestamp. Unknown inputs produce Unknown
//! findings (never Good). Rules are pure functions over real collected data.

use serde::{Deserialize, Serialize};

use crate::model::{Confidence, Severity};

/// A single diagnostic finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub category: String,
    pub finding: String,
    pub severity: Severity,
    pub evidence: String,
    pub data_source: String,
    pub explanation: String,
    pub recommendation: String,
    pub confidence: Confidence,
    pub timestamp_ms: u64,
}

impl Finding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        category: &str,
        finding: &str,
        severity: Severity,
        evidence: &str,
        data_source: &str,
        explanation: &str,
        recommendation: &str,
        confidence: Confidence,
    ) -> Self {
        Self {
            id: id.into(),
            category: category.into(),
            finding: finding.into(),
            severity,
            evidence: evidence.into(),
            data_source: data_source.into(),
            explanation: explanation.into(),
            recommendation: recommendation.into(),
            confidence,
            timestamp_ms: 0,
        }
    }

    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp_ms = ts;
        self
    }
}

/// Real inputs to the rule engine. `None` means the value was not available.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticInput {
    pub battery_health_pct: Option<f64>,
    pub battery_present: bool,
    pub storage_severity: Option<Severity>,
    pub storage_label: String,
    pub memory_used_pct: Option<f64>,
    pub cpu_temp_c: Option<f64>,
    pub gpu_temp_c: Option<f64>,
    pub storage_temp_c: Option<f64>,
    pub driver_problem_count: Option<u32>,
    pub critical_event_count: Option<u32>,
    pub network_connected: Option<bool>,
    pub failed_test_count: u32,
}

/// Documented temperature thresholds (°C). These are general guidance, not
/// manufacturer-specific limits; the note in each finding says so.
pub const CPU_TEMP_WARN: f64 = 85.0;
pub const CPU_TEMP_CRIT: f64 = 95.0;
pub const GPU_TEMP_WARN: f64 = 85.0;
pub const GPU_TEMP_CRIT: f64 = 95.0;
pub const STORAGE_TEMP_WARN: f64 = 60.0;
pub const STORAGE_TEMP_CRIT: f64 = 70.0;
pub const MEMORY_USED_WARN: f64 = 90.0;

/// Evaluate all rules and return findings sorted by severity (worst first).
pub fn evaluate(input: &DiagnosticInput) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Battery rule.
    if input.battery_present {
        match input.battery_health_pct {
            Some(h) => {
                let sev = crate::battery::health_severity(h);
                let (finding, recommendation) = match sev {
                    Severity::Good => ("Battery health is good", "No action needed. Avoid sustained full charges and high heat to preserve longevity."),
                    Severity::Information => ("Battery shows mild wear", "Monitor over time; avoid deep discharges and heat."),
                    Severity::Warning => ("Battery health is degraded", "Expect reduced runtime. Consider calibration; plan for replacement if runtime is insufficient."),
                    Severity::Critical => ("Battery health is poor", "Runtime is likely severely reduced. Battery replacement is recommended."),
                    Severity::Unknown => ("Battery health could not be determined", "Ensure the battery is detected and drivers are current."),
                };
                findings.push(Finding::new(
                    "battery_health", "Battery", finding, sev,
                    &format!("Full-charge/design capacity ratio = {h:.1}%"),
                    "Battery capacity interface (IOCTL_BATTERY_QUERY_INFORMATION / WMI)",
                    "Health = full-charge capacity / design capacity × 100. Thresholds: ≥80 good, 60–79 mild wear, 40–59 degraded, <40 poor.",
                    recommendation, Confidence::High,
                ));
            }
            None => findings.push(Finding::new(
                "battery_health", "Battery", "Battery health could not be computed", Severity::Unknown,
                "Design or full-charge capacity unavailable",
                "Battery capacity interface", "Both capacities are required for a valid health calculation.",
                "Check that the battery is properly seated and the chipset/battery drivers are installed.", Confidence::Low,
            )),
        }
    }

    // Storage rule.
    match input.storage_severity {
        Some(sev) => {
            let label = if input.storage_label.is_empty() { "Storage device" } else { &input.storage_label };
            let (finding, recommendation) = match sev {
                Severity::Good => (format!("{label}: SMART health is good"), "No action needed. Keep regular backups."),
                Severity::Information => (format!("{label}: minor SMART indicators"), "Monitor periodically; ensure backups are current."),
                Severity::Warning => (format!("{label}: SMART warnings present"), "Back up important data now and monitor closely; consider replacement soon."),
                Severity::Critical => (format!("{label}: SMART indicates failure risk"), "Back up data immediately and replace the drive. Failure may be imminent."),
                Severity::Unknown => (format!("{label}: health unknown"), "SMART data was not available; health cannot be confirmed."),
            };
            findings.push(Finding::new(
                "storage_health", "Storage", &finding, sev,
                "SMART attributes / NVMe health log", "Storage SMART interface (ATA attributes / NVMe health log / WMI)",
                "Interpretation uses vendor thresholds and spec-defined fields, not a universal cutoff.",
                recommendation, if sev == Severity::Unknown { Confidence::Low } else { Confidence::High },
            ));
        }
        None => findings.push(Finding::new(
            "storage_health", "Storage", "Storage health unknown", Severity::Unknown,
            "No SMART data available", "Storage SMART interface",
            "SMART was not exposed or accessible for the detected devices.",
            "Some devices/enclosures do not expose SMART. Administrator access may be required for ATA SMART.", Confidence::Low,
        )),
    }

    // Memory pressure rule.
    if let Some(used) = input.memory_used_pct {
        let sev = if used >= MEMORY_USED_WARN {
            Severity::Warning
        } else {
            Severity::Good
        };
        findings.push(Finding::new(
            "memory_pressure",
            "Memory",
            if sev == Severity::Warning {
                "High memory pressure"
            } else {
                "Memory availability is healthy"
            },
            sev,
            &format!("{used:.1}% of RAM in use"),
            "sysinfo / GlobalMemoryStatusEx",
            &format!("Sustained usage above {MEMORY_USED_WARN}% can cause paging and slowdowns."),
            if sev == Severity::Warning {
                "Close unused applications or consider adding RAM."
            } else {
                "No action needed."
            },
            Confidence::High,
        ));
    }

    // Thermal rules.
    push_temp(&mut findings, "cpu_temp", "CPU", input.cpu_temp_c, CPU_TEMP_WARN, CPU_TEMP_CRIT,
        "CPU temperature sensor (sysinfo / WMI thermal zone)",
        "Improve cooling: clean vents/fans, ensure airflow, reapply thermal paste if comfortable. General guidance, not a manufacturer limit.");
    push_temp(&mut findings, "gpu_temp", "GPU", input.gpu_temp_c, GPU_TEMP_WARN, GPU_TEMP_CRIT,
        "GPU temperature sensor (vendor interface when available)",
        "Improve case airflow and clean the GPU cooler. General guidance, not a manufacturer limit.");
    push_temp(
        &mut findings,
        "storage_temp",
        "Storage",
        input.storage_temp_c,
        STORAGE_TEMP_WARN,
        STORAGE_TEMP_CRIT,
        "Drive temperature (SMART / NVMe health log)",
        "Improve drive cooling/airflow. Sustained high temperatures reduce SSD/HDD lifespan.",
    );

    // Driver problems.
    if let Some(n) = input.driver_problem_count {
        let sev = if n > 0 {
            Severity::Warning
        } else {
            Severity::Good
        };
        findings.push(Finding::new(
            "driver_problems",
            "Drivers",
            if n > 0 {
                "Devices with driver problems detected"
            } else {
                "No driver problems detected"
            },
            sev,
            &format!("{n} device(s) reporting a problem"),
            "WMI Win32_PnPEntity (ConfigManagerErrorCode)",
            "Devices with a non-zero ConfigManagerErrorCode may be malfunctioning.",
            if n > 0 {
                "Review the devices in Windows Diagnostics and update/reinstall drivers manually."
            } else {
                "No action needed."
            },
            Confidence::Medium,
        ));
    }

    // System stability (critical events).
    if let Some(n) = input.critical_event_count {
        let sev = if n > 0 {
            Severity::Warning
        } else {
            Severity::Good
        };
        findings.push(Finding::new(
            "system_stability",
            "Stability",
            if n > 0 {
                "Critical system events recorded"
            } else {
                "No recent critical system events"
            },
            sev,
            &format!("{n} critical event(s) in the recent log window"),
            "Windows Event Log (System)",
            "Critical events (e.g. unexpected shutdowns, bugchecks) can indicate instability.",
            if n > 0 {
                "Review the events in Windows Diagnostics; investigate recurring bugchecks."
            } else {
                "No action needed."
            },
            Confidence::Medium,
        ));
    }

    // Network.
    if let Some(connected) = input.network_connected {
        findings.push(Finding::new(
            "network_connectivity",
            "Network",
            if connected {
                "Network connectivity present"
            } else {
                "No network connectivity detected"
            },
            if connected {
                Severity::Good
            } else {
                Severity::Information
            },
            if connected {
                "At least one active interface"
            } else {
                "No active interface detected"
            },
            "sysinfo network interfaces",
            "Based on interface operational state.",
            if connected {
                "No action needed."
            } else {
                "Check the adapter, cable/Wi-Fi and router if connectivity is expected."
            },
            Confidence::Medium,
        ));
    }

    // Failed tests.
    if input.failed_test_count > 0 {
        findings.push(Finding::new(
            "test_failures",
            "Testing",
            "One or more hardware tests failed",
            Severity::Critical,
            &format!("{} test(s) reported failure", input.failed_test_count),
            "VYRONIX test results",
            "A failed test indicates a real problem detected during testing.",
            "Review the individual test results and re-test the affected component.",
            Confidence::High,
        ));
    }

    findings.sort_by_key(|f| std::cmp::Reverse(f.severity.rank()));
    findings
}

#[allow(clippy::too_many_arguments)]
fn push_temp(
    findings: &mut Vec<Finding>,
    id: &str,
    category: &str,
    temp: Option<f64>,
    warn: f64,
    crit: f64,
    source: &str,
    recommendation: &str,
) {
    let Some(t) = temp else {
        findings.push(Finding::new(
            id, category, &format!("{category} temperature unavailable"), Severity::Unknown,
            "No sensor reading available", source,
            "This system does not expose the sensor through a supported interface, or access is restricted.",
            "Temperature cannot be assessed. Vendor-specific software may expose it.", Confidence::Low,
        ));
        return;
    };
    let sev = if t >= crit {
        Severity::Critical
    } else if t >= warn {
        Severity::Warning
    } else {
        Severity::Good
    };
    findings.push(Finding::new(
        id, category,
        &format!("{category} temperature is {}", match sev { Severity::Critical => "critical", Severity::Warning => "elevated", _ => "normal" }),
        sev, &format!("{t:.1} °C"), source,
        &format!("General guidance thresholds: warning ≥{warn}°C, critical ≥{crit}°C. Not a manufacturer-specific limit."),
        if sev == Severity::Good { "No action needed." } else { recommendation },
        Confidence::Medium,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_battery_not_good() {
        let input = DiagnosticInput {
            battery_present: true,
            battery_health_pct: None,
            ..Default::default()
        };
        let findings = evaluate(&input);
        let b = findings.iter().find(|f| f.id == "battery_health").unwrap();
        assert_eq!(b.severity, Severity::Unknown);
    }

    #[test]
    fn good_battery() {
        let input = DiagnosticInput {
            battery_present: true,
            battery_health_pct: Some(92.0),
            ..Default::default()
        };
        let findings = evaluate(&input);
        let b = findings.iter().find(|f| f.id == "battery_health").unwrap();
        assert_eq!(b.severity, Severity::Good);
        assert!(b.evidence.contains("92.0%"));
    }

    #[test]
    fn critical_storage_sorts_first() {
        let input = DiagnosticInput {
            storage_severity: Some(Severity::Critical),
            storage_label: "NVMe SSD".into(),
            battery_present: true,
            battery_health_pct: Some(95.0),
            ..Default::default()
        };
        let findings = evaluate(&input);
        assert_eq!(findings[0].severity, Severity::Critical);
        assert_eq!(findings[0].id, "storage_health");
    }

    #[test]
    fn temp_unavailable_is_unknown() {
        let input = DiagnosticInput::default();
        let findings = evaluate(&input);
        let cpu = findings.iter().find(|f| f.id == "cpu_temp").unwrap();
        assert_eq!(cpu.severity, Severity::Unknown);
    }

    #[test]
    fn temp_thresholds() {
        let input = DiagnosticInput {
            cpu_temp_c: Some(96.0),
            ..Default::default()
        };
        let findings = evaluate(&input);
        let cpu = findings.iter().find(|f| f.id == "cpu_temp").unwrap();
        assert_eq!(cpu.severity, Severity::Critical);
    }

    #[test]
    fn every_finding_has_required_fields() {
        let input = DiagnosticInput {
            battery_present: true,
            battery_health_pct: Some(70.0),
            storage_severity: Some(Severity::Good),
            storage_label: "SSD".into(),
            memory_used_pct: Some(50.0),
            cpu_temp_c: Some(50.0),
            driver_problem_count: Some(0),
            critical_event_count: Some(0),
            network_connected: Some(true),
            failed_test_count: 0,
            ..Default::default()
        };
        for f in evaluate(&input) {
            assert!(!f.finding.is_empty());
            assert!(!f.evidence.is_empty());
            assert!(!f.data_source.is_empty());
            assert!(!f.explanation.is_empty());
            assert!(!f.recommendation.is_empty());
        }
    }

    #[test]
    fn failed_tests_critical() {
        let input = DiagnosticInput {
            failed_test_count: 2,
            ..Default::default()
        };
        let findings = evaluate(&input);
        let t = findings.iter().find(|f| f.id == "test_failures").unwrap();
        assert_eq!(t.severity, Severity::Critical);
    }
}
