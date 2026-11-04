import { useState } from "react";
import { FileText, Download, Shield } from "lucide-react";
import { Card, PageHeader } from "../components/Card";
import { Spinner } from "../components/ProgressBar";
import { SeverityBadge } from "../components/StatusBadge";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";
import {
  buildReport, saveReportFile, pickSavePath, type ReportType, type ReportFormat,
} from "../lib/services";
import { generateReportPdf } from "../lib/pdf";
import type { ReportBundle } from "../lib/types";
import { timestampToLocale } from "../lib/format";

const REPORT_TYPES: { value: ReportType; label: string }[] = [
  { value: "full", label: "Full system report" },
  { value: "hardware", label: "Hardware report" },
  { value: "battery", label: "Battery report" },
  { value: "storage", label: "Storage health report" },
  { value: "benchmark", label: "Benchmark report" },
  { value: "inspection", label: "Device inspection report" },
  { value: "summary", label: "Diagnostic summary" },
];

const FORMATS: { value: ReportFormat; label: string }[] = [
  { value: "pdf", label: "PDF" },
  { value: "html", label: "HTML" },
  { value: "csv", label: "CSV" },
  { value: "json", label: "JSON" },
];

export function Reports() {
  const settings = useAppStore((s) => s.settings);
  const updateSettings = useAppStore((s) => s.updateSettings);
  const pushToast = useAppStore((s) => s.pushToast);
  const [reportType, setReportType] = useState<ReportType>("full");
  const [format, setFormat] = useState<ReportFormat>(settings.report_default_format);
  const [includeSensitive, setIncludeSensitive] = useState(settings.include_sensitive_in_reports);
  const [busy, setBusy] = useState(false);
  const [preview, setPreview] = useState<ReportBundle | null>(null);

  const generate = async (exportAfter: boolean) => {
    if (!isNative) {
      pushToast({ kind: "warning", title: "Desktop app required", message: "Reports are built from real hardware data in the Windows desktop application." });
      return;
    }
    setBusy(true);
    try {
      const bundle = await buildReport(reportType, includeSensitive);
      setPreview(bundle);
      if (exportAfter) await exportBundle(bundle);
    } catch (e) {
      pushToast({ kind: "error", title: "Report generation failed", message: e instanceof Error ? e.message : String(e) });
    } finally {
      setBusy(false);
    }
  };

  const exportBundle = async (bundle: ReportBundle) => {
    const stamp = new Date().toISOString().slice(0, 19).replace(/[:T]/g, "-");
    const defaultName = `VYRONIX-${reportType}-${stamp}.${format}`;
    const path = await pickSavePath(defaultName, format);
    if (!path) return; // user cancelled
    let contents: string;
    let isBase64 = false;
    if (format === "pdf") { contents = generateReportPdf(bundle.model); isBase64 = true; }
    else if (format === "html") contents = bundle.html;
    else if (format === "csv") contents = bundle.csv;
    else contents = bundle.json;
    try {
      await saveReportFile(path, contents, isBase64, false);
      pushToast({ kind: "success", title: "Report exported", message: path });
    } catch (e) {
      pushToast({ kind: "error", title: "Export failed", message: e instanceof Error ? e.message : String(e) });
    }
  };

  return (
    <div>
      <PageHeader title="Reports & Export" description="Professional, branded reports — generated offline from real collected data" />

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <Card title="Configure report" className="lg:col-span-1">
          <div className="space-y-4">
            <div>
              <label className="vx-label">Report type</label>
              <select className="vx-input mt-1" value={reportType} onChange={(e) => setReportType(e.target.value as ReportType)}>
                {REPORT_TYPES.map((t) => <option key={t.value} value={t.value}>{t.label}</option>)}
              </select>
            </div>
            <div>
              <label className="vx-label">Format</label>
              <div className="grid grid-cols-4 gap-1 mt-1">
                {FORMATS.map((f) => (
                  <button key={f.value} onClick={() => setFormat(f.value)}
                    className={`px-2 py-2 rounded-lg text-xs font-medium border transition ${format === f.value ? "bg-vx-600/15 border-vx-600/40 text-vx-500" : "border-[var(--vx-border)] hover:bg-vx-500/10"}`}>
                    {f.label}
                  </button>
                ))}
              </div>
            </div>
            <label className="flex items-start gap-2 cursor-pointer">
              <input type="checkbox" className="mt-0.5 accent-vx-500" checked={includeSensitive}
                onChange={(e) => { setIncludeSensitive(e.target.checked); updateSettings({ include_sensitive_in_reports: e.target.checked }); }} />
              <span className="text-xs text-[var(--vx-muted)]">
                Include sensitive identifiers (serial numbers, MAC/IP addresses). Off by default to protect privacy.
              </span>
            </label>
            <div className="flex flex-col gap-2">
              <button className="vx-btn-primary" onClick={() => generate(true)} disabled={busy}>
                {busy ? <Spinner size={16} /> : <Download className="w-4 h-4" />} Generate & export
              </button>
              <button className="vx-btn-ghost" onClick={() => generate(false)} disabled={busy}>
                <FileText className="w-4 h-4" /> Preview only
              </button>
            </div>
            <p className="text-[11px] text-[var(--vx-muted)] flex items-start gap-1.5">
              <Shield className="w-3.5 h-3.5 shrink-0 mt-0.5 text-emerald-500" />
              Exports work fully offline. You choose the destination, and existing files are protected from accidental overwrite.
            </p>
          </div>
        </Card>

        <Card title="Preview" className="lg:col-span-2">
          {!preview ? (
            <div className="flex flex-col items-center justify-center py-12 text-[var(--vx-muted)]">
              <FileText className="w-8 h-8 mb-3 opacity-50" />
              <span className="text-sm">Generate a report to preview its findings and score.</span>
            </div>
          ) : (
            <div>
              <div className="flex items-center justify-between mb-3">
                <div>
                  <div className="text-sm font-semibold">{preview.model.title}</div>
                  <div className="text-[11px] text-[var(--vx-muted)]">{timestampToLocale(preview.model.generated_at_ms)} · {preview.model.device.device_name}</div>
                </div>
                {preview.model.overall?.score != null && (
                  <div className="text-2xl font-bold text-vx-500 tabular-nums">{Math.round(preview.model.overall.score)}<span className="text-sm text-[var(--vx-muted)]">/100</span></div>
                )}
              </div>
              <div className="space-y-2 max-h-[400px] overflow-y-auto">
                {preview.model.findings.length === 0 ? (
                  <p className="text-sm text-[var(--vx-muted)]">No findings for this report type.</p>
                ) : preview.model.findings.map((f) => (
                  <div key={f.id} className="rounded-lg border border-[var(--vx-border)] p-3">
                    <div className="flex items-center justify-between gap-2">
                      <span className="text-sm font-medium">{f.finding}</span>
                      <SeverityBadge severity={f.severity} />
                    </div>
                    <div className="text-xs text-[var(--vx-muted)] mt-1">{f.evidence}</div>
                    <div className="text-xs text-vx-500 mt-1">{f.recommendation}</div>
                  </div>
                ))}
              </div>
              <button className="vx-btn-ghost mt-3" onClick={() => exportBundle(preview)}>
                <Download className="w-4 h-4" /> Export this report as {format.toUpperCase()}
              </button>
            </div>
          )}
        </Card>
      </div>
    </div>
  );
}
