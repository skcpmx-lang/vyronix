import { useEffect, useRef, useState } from "react";
import { Play, Pause, Activity } from "lucide-react";
import { getMonitoringSample } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { LiveChart } from "../components/LiveChart";
import { NativeRequired } from "../components/NativeRequired";
import { formatPct, formatTemp, formatMBs, formatMbps } from "../lib/format";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";
import type { MonitoringSample } from "../lib/types";

const WINDOW = 60;

interface SeriesStats { min: number | null; max: number | null; avg: number | null; current: number | null; }

function stats(history: (number | null)[]): SeriesStats {
  const vals = history.filter((v): v is number => v !== null && isFinite(v));
  if (!vals.length) return { min: null, max: null, avg: null, current: null };
  return {
    min: Math.min(...vals),
    max: Math.max(...vals),
    avg: vals.reduce((a, b) => a + b, 0) / vals.length,
    current: vals[vals.length - 1],
  };
}

export function MonitoringPage() {
  const interval = useAppStore((s) => s.settings.monitoring_interval_ms);
  const updateSettings = useAppStore((s) => s.updateSettings);
  const [active, setActive] = useState(true);
  const [history, setHistory] = useState<MonitoringSample[]>([]);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (!active || !isNative) return;
    let cancelled = false;
    const tick = async () => {
      try {
        const s = await getMonitoringSample();
        if (!cancelled && s) setHistory((h) => [...h.slice(-(WINDOW - 1)), s]);
      } catch { /* non-fatal */ }
      if (!cancelled) timer.current = setTimeout(tick, interval);
    };
    tick();
    return () => { cancelled = true; if (timer.current) clearTimeout(timer.current); };
  }, [active, interval]);

  const series = (key: keyof MonitoringSample) => history.map((h) => {
    const v = h[key];
    return typeof v === "number" ? v : null;
  });

  const charts: { title: string; data: (number | null)[]; unit: string; color: string; max?: number; fmt: (v: number | null) => string }[] = [
    { title: "CPU usage", data: series("cpu_usage_pct"), unit: "%", color: "#3391ff", max: 100, fmt: formatPct },
    { title: "Memory usage", data: series("memory_used_pct"), unit: "%", color: "#22d3ee", max: 100, fmt: formatPct },
    { title: "CPU temperature", data: series("cpu_temp_c"), unit: "°C", color: "#f59e0b", fmt: formatTemp },
    { title: "GPU usage", data: series("gpu_usage_pct"), unit: "%", color: "#a78bfa", max: 100, fmt: formatPct },
    { title: "Disk read", data: series("disk_read_bytes_per_s"), unit: "", color: "#10b981", fmt: (v) => formatMBs(v) },
    { title: "Disk write", data: series("disk_write_bytes_per_s"), unit: "", color: "#f43f5e", fmt: (v) => formatMBs(v) },
    { title: "Network receive", data: series("net_rx_bytes_per_s"), unit: "", color: "#0ea5e9", fmt: (v) => formatMbps(v) },
    { title: "Network transmit", data: series("net_tx_bytes_per_s"), unit: "", color: "#8b5cf6", fmt: (v) => formatMbps(v) },
  ];

  return (
    <div>
      <PageHeader title="Live Monitoring" description="Real-time hardware sensors with rolling history and statistics"
        actions={
          <div className="flex items-center gap-2">
            <select className="vx-input w-auto" value={interval} aria-label="Polling interval"
              onChange={(e) => updateSettings({ monitoring_interval_ms: Number(e.target.value) })}>
              <option value={500}>500 ms</option>
              <option value={1000}>1 s</option>
              <option value={2000}>2 s</option>
              <option value={5000}>5 s</option>
            </select>
            <button className="vx-btn-ghost" onClick={() => setActive((a) => !a)}>
              {active ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
              {active ? "Pause" : "Resume"}
            </button>
          </div>
        } />
      <NativeRequired what="Live monitoring" />

      {!isNative ? (
        <div className="mt-4 vx-panel p-10 text-center text-sm text-[var(--vx-muted)]">
          <Activity className="w-8 h-8 mx-auto mb-3 opacity-50" />
          Live sensor graphs appear here in the Windows desktop application. No simulated readings are shown.
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
          {charts.map((c) => {
            const st = stats(c.data);
            return (
              <Card key={c.title} title={c.title}>
                <LiveChart data={c.data} max={c.max} unit={c.unit} color={c.color} height={90} />
                <div className="grid grid-cols-4 gap-2 mt-3 text-center">
                  {[["Min", st.min], ["Avg", st.avg], ["Max", st.max], ["Now", st.current]].map(([label, v]) => (
                    <div key={label as string}>
                      <div className="text-[10px] uppercase tracking-wider text-[var(--vx-muted)]">{label}</div>
                      <div className="text-xs font-semibold tabular-nums">{c.fmt(v as number | null)}</div>
                    </div>
                  ))}
                </div>
              </Card>
            );
          })}
        </div>
      )}
    </div>
  );
}
