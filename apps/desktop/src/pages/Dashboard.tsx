import { useMemo } from "react";
import {
  BatteryCharging, HardDrive, Cpu, MemoryStick, Thermometer, Clock,
  ScanLine, Gauge, ClipboardCheck, FileText, AlertTriangle, ChevronRight,
} from "lucide-react";
import { useAsync, usePoll } from "../hooks/useAsync";
import {
  runDiagnostics, getMonitoringSample, getBatteryInfo, getStorageDevices, getSystemOverview,
} from "../lib/services";
import { useAppStore, type PageId } from "../lib/store";
import { Card, PageHeader } from "../components/Card";
import { Gauge as GaugeRing } from "../components/Gauge";
import { SeverityBadge, severityDot } from "../components/StatusBadge";
import { NativeRequired } from "../components/NativeRequired";
import { Spinner } from "../components/ProgressBar";
import { formatBytes, formatDuration, formatPct, formatTemp } from "../lib/format";
import { isNative } from "../lib/ipc";

export function Dashboard() {
  const setPage = useAppStore((s) => s.setPage);
  const interval = useAppStore((s) => s.settings.monitoring_interval_ms);

  const diag = useAsync(runDiagnostics, []);
  const battery = useAsync(getBatteryInfo, []);
  const storage = useAsync(getStorageDevices, []);
  const system = useAsync(getSystemOverview, []);
  const sample = usePoll(getMonitoringSample, interval, isNative);

  const criticalFindings = useMemo(
    () => (diag.data?.findings ?? []).filter((f) => f.severity === "critical" || f.severity === "warning"),
    [diag.data]
  );

  const storageWorst = useMemo(() => {
    const devs = storage.data ?? [];
    if (!devs.length) return null;
    const rank = { critical: 4, warning: 3, unknown: 2, information: 1, good: 0 } as const;
    return devs.reduce((a, b) => (rank[b.severity] > rank[a.severity] ? b : a));
  }, [storage.data]);

  const quickActions: { label: string; icon: typeof ScanLine; page: PageId; desc: string }[] = [
    { label: "Full diagnostic scan", icon: ScanLine, page: "reports", desc: "Evaluate all components" },
    { label: "Battery test", icon: BatteryCharging, page: "battery", desc: "Health & wear analysis" },
    { label: "Storage test", icon: HardDrive, page: "storage", desc: "SMART health & benchmark" },
    { label: "Hardware inspection", icon: ClipboardCheck, page: "inspection", desc: "Guided device checklist" },
    { label: "Performance benchmark", icon: Gauge, page: "benchmarks", desc: "CPU, memory & storage" },
    { label: "Generate report", icon: FileText, page: "reports", desc: "Export PDF / HTML / CSV / JSON" },
  ];

  return (
    <div>
      <PageHeader
        title="Dashboard"
        description={system.data?.device_name.value ? `Device: ${system.data.device_name.value}` : "Overall device condition and live metrics"}
        actions={
          <button className="vx-btn-primary" onClick={() => diag.refresh()}>
            <ScanLine className="w-4 h-4" /> Run full scan
          </button>
        }
      />

      <NativeRequired what="The dashboard" />

      {/* Top row: overall score + key health widgets */}
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4 mt-4">
        <Card className="flex flex-col items-center justify-center text-center">
          {diag.loading ? <Spinner /> : (
            <>
              <GaugeRing value={diag.data?.overall.score ?? null} label="Condition" size={130} />
              <div className="text-xs text-[var(--vx-muted)] mt-2">
                {diag.data
                  ? `Coverage ${Math.round(diag.data.overall.coverage * 100)}% · ${diag.data.overall.confidence} confidence`
                  : "Run a scan to assess"}
              </div>
            </>
          )}
        </Card>

        <WidgetCard
          title="Battery health" icon={<BatteryCharging className="w-4 h-4" />}
          loading={battery.loading}
          primary={battery.data?.present ? formatPct(battery.data.health_pct.value) : "No battery"}
          secondary={battery.data?.present ? `Wear ${formatPct(battery.data.wear_pct.value)}` : "Desktop / no battery detected"}
          onClick={() => setPage("battery")}
        />
        <WidgetCard
          title="Storage health" icon={<HardDrive className="w-4 h-4" />}
          loading={storage.loading}
          primary={storageWorst ? storageWorst.severity.toUpperCase() : "—"}
          secondary={storageWorst?.model.value ?? "No devices"}
          onClick={() => setPage("storage")}
        />
        <WidgetCard
          title="Uptime" icon={<Clock className="w-4 h-4" />}
          loading={system.loading}
          primary={formatDuration(system.data?.uptime_secs.value ?? null)}
          secondary={system.data?.os_edition.value ?? "—"}
          onClick={() => setPage("system")}
        />
      </div>

      {/* Live metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4 mt-4">
        <LiveMetric title="CPU usage" icon={<Cpu className="w-4 h-4" />} value={formatPct(sample?.cpu_usage_pct ?? null)} />
        <LiveMetric title="Memory usage" icon={<MemoryStick className="w-4 h-4" />} value={formatPct(sample?.memory_used_pct ?? null)}
          sub={sample?.memory_used_bytes != null ? formatBytes(sample.memory_used_bytes) : undefined} />
        <LiveMetric title="CPU temperature" icon={<Thermometer className="w-4 h-4" />} value={formatTemp(sample?.cpu_temp_c ?? null)} />
        <LiveMetric title="GPU usage" icon={<Gauge className="w-4 h-4" />} value={formatPct(sample?.gpu_usage_pct ?? null)} />
      </div>

      {/* Alerts + quick actions */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4">
        <Card title="Important alerts" subtitle={`${criticalFindings.length} finding(s) need attention`}>
          {diag.loading ? <Spinner /> : criticalFindings.length === 0 ? (
            <p className="text-sm text-[var(--vx-muted)]">
              {diag.data ? "No warnings or critical findings. Unknown values are not counted as good." : "Run a scan to see findings."}
            </p>
          ) : (
            <div className="space-y-2">
              {criticalFindings.slice(0, 5).map((f) => (
                <div key={f.id} className="flex items-start gap-3 py-2 border-b border-[var(--vx-border)] last:border-0">
                  <span className={`w-2 h-2 rounded-full mt-1.5 shrink-0 ${severityDot(f.severity)}`} />
                  <div className="min-w-0 flex-1">
                    <div className="text-sm font-medium">{f.finding}</div>
                    <div className="text-xs text-[var(--vx-muted)] truncate">{f.evidence}</div>
                  </div>
                  <SeverityBadge severity={f.severity} />
                </div>
              ))}
            </div>
          )}
        </Card>

        <Card title="Quick actions" subtitle="Jump straight into a tool">
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
            {quickActions.map((a) => (
              <button key={a.label} onClick={() => setPage(a.page)}
                className="flex items-center gap-3 px-3 py-2.5 rounded-lg border border-[var(--vx-border)] hover:bg-vx-500/10 hover:border-vx-500/40 transition text-left">
                <a.icon className="w-4 h-4 text-vx-500 shrink-0" />
                <div className="min-w-0 flex-1">
                  <div className="text-sm font-medium truncate">{a.label}</div>
                  <div className="text-[11px] text-[var(--vx-muted)] truncate">{a.desc}</div>
                </div>
                <ChevronRight className="w-3.5 h-3.5 text-[var(--vx-muted)]" />
              </button>
            ))}
          </div>
        </Card>
      </div>

      {diag.error && (
        <div className="mt-4 vx-panel p-4 flex items-center gap-3 border-red-500/30">
          <AlertTriangle className="w-4 h-4 text-red-500" />
          <span className="text-sm text-[var(--vx-muted)]">Diagnostic scan unavailable: {diag.error}</span>
        </div>
      )}
    </div>
  );
}

function WidgetCard({ title, icon, primary, secondary, loading, onClick }: {
  title: string; icon: React.ReactNode; primary: string; secondary?: string; loading: boolean; onClick: () => void;
}) {
  return (
    <button onClick={onClick} className="vx-panel p-5 text-left hover:border-vx-500/40 transition animate-fade-in">
      <div className="flex items-center gap-2 text-[var(--vx-muted)] text-xs font-semibold uppercase tracking-wider">
        {icon} {title}
      </div>
      {loading ? <div className="mt-3"><Spinner size={18} /></div> : (
        <>
          <div className="text-2xl font-bold mt-2 tabular-nums">{primary}</div>
          {secondary && <div className="text-xs text-[var(--vx-muted)] mt-1 truncate">{secondary}</div>}
        </>
      )}
    </button>
  );
}

function LiveMetric({ title, icon, value, sub }: { title: string; icon: React.ReactNode; value: string; sub?: string }) {
  return (
    <div className="vx-panel p-4 animate-fade-in">
      <div className="flex items-center gap-2 text-[var(--vx-muted)] text-xs font-semibold uppercase tracking-wider">
        {icon} {title}
      </div>
      <div className="text-xl font-bold mt-2 tabular-nums">{value}</div>
      {sub && <div className="text-[11px] text-[var(--vx-muted)] mt-0.5">{sub}</div>}
    </div>
  );
}
