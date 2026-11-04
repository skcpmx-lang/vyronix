import { useEffect, useState } from "react";
import { Flame, Play, Pause, Square, AlertTriangle } from "lucide-react";
import { Card, PageHeader } from "../components/Card";
import { ConfirmDialog } from "../components/Modal";
import { ProgressBar, Spinner } from "../components/ProgressBar";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";
import { startStressTest, stopStressTest, pauseStressTest, getStressStatus } from "../lib/services";
import type { StressStatus } from "../lib/types";
import { formatPct, formatTemp, formatDuration } from "../lib/format";

export function StressTest() {
  const pushToast = useAppStore((s) => s.pushToast);
  const [duration, setDuration] = useState(60);
  const [tempLimit, setTempLimit] = useState(90);
  const [status, setStatus] = useState<StressStatus | null>(null);
  const [confirmOpen, setConfirmOpen] = useState(false);

  useEffect(() => {
    if (!isNative) return;
    const t = setInterval(async () => {
      try { setStatus(await getStressStatus()); } catch { /* non-fatal */ }
    }, 1000);
    return () => clearInterval(t);
  }, []);

  const start = async () => {
    if (!isNative) {
      pushToast({ kind: "warning", title: "Desktop app required", message: "Stress testing runs in the Windows desktop application." });
      return;
    }
    try {
      await startStressTest(duration, tempLimit);
      pushToast({ kind: "info", title: "Stress test started", message: `Running for up to ${duration}s with a ${tempLimit}°C safety limit.` });
    } catch (e) { pushToast({ kind: "error", title: "Could not start stress test", message: String(e) }); }
  };

  const running = status?.running ?? false;
  const progress = status && status.target_secs > 0 ? (status.elapsed_secs / status.target_secs) * 100 : 0;

  return (
    <div>
      <PageHeader title="CPU Stress Test" description="Controlled load with configurable safety limits and live telemetry" />

      <div className="vx-panel p-4 border-amber-500/30 bg-amber-500/5 flex items-start gap-3 mb-4">
        <AlertTriangle className="w-5 h-5 text-amber-500 shrink-0 mt-0.5" />
        <div className="text-xs text-[var(--vx-muted)] leading-relaxed">
          <strong className="text-amber-500">Warning:</strong> A stress test generates sustained high CPU load and heat.
          Ensure your cooling is working and the device is on a hard, ventilated surface. VYRONIX stops automatically when
          the configured temperature limit or duration is reached, but temperature alone cannot guarantee safety — never
          leave a stress test unattended, and stop immediately if you notice unusual noise, smell or behavior. No test is
          unlimited by default, and hardware protection mechanisms are never bypassed.
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <Card title="Configuration">
          <div className="space-y-4">
            <div>
              <label className="vx-label">Duration (seconds)</label>
              <select className="vx-input mt-1" value={duration} onChange={(e) => setDuration(Number(e.target.value))} disabled={running}>
                {[30, 60, 120, 300, 600].map((d) => <option key={d} value={d}>{formatDuration(d)}</option>)}
              </select>
            </div>
            <div>
              <label className="vx-label">Temperature safety limit: {tempLimit}°C</label>
              <input type="range" min={60} max={100} value={tempLimit} onChange={(e) => setTempLimit(Number(e.target.value))}
                disabled={running} className="w-full mt-2 accent-vx-500" aria-label="Temperature limit" />
              <p className="text-[11px] text-[var(--vx-muted)] mt-1">The test stops if a reported CPU temperature reaches this limit. If no sensor is available, the duration limit still applies.</p>
            </div>
            <div className="flex gap-2">
              {!running ? (
                <button className="vx-btn-primary" onClick={() => setConfirmOpen(true)}><Flame className="w-4 h-4" /> Start stress test</button>
              ) : (
                <>
                  <button className="vx-btn-ghost" onClick={() => pauseStressTest(!(status?.paused ?? false))}>
                    {status?.paused ? <Play className="w-4 h-4" /> : <Pause className="w-4 h-4" />}
                    {status?.paused ? "Resume" : "Pause"}
                  </button>
                  <button className="vx-btn bg-red-600 text-white hover:bg-red-500" onClick={() => stopStressTest()}>
                    <Square className="w-4 h-4" /> Stop
                  </button>
                </>
              )}
            </div>
          </div>
        </Card>

        <Card title="Live status">
          {!isNative ? (
            <p className="text-sm text-[var(--vx-muted)]">Live stress telemetry appears in the Windows desktop application.</p>
          ) : !status ? (
            <div className="flex items-center gap-2 text-sm text-[var(--vx-muted)]"><Spinner size={16} /> Waiting for status…</div>
          ) : (
            <div className="space-y-4">
              <div>
                <div className="flex justify-between text-xs text-[var(--vx-muted)] mb-1">
                  <span>{running ? (status.paused ? "Paused" : "Running") : "Idle"}</span>
                  <span>{formatDuration(status.elapsed_secs)} / {formatDuration(status.target_secs)}</span>
                </div>
                <ProgressBar value={progress} color={status.paused ? "#f59e0b" : "#ef4444"} />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div className="rounded-lg border border-[var(--vx-border)] p-3 text-center">
                  <div className="text-xl font-bold tabular-nums">{formatPct(status.cpu_usage_pct)}</div>
                  <div className="text-[11px] text-[var(--vx-muted)]">CPU usage</div>
                </div>
                <div className="rounded-lg border border-[var(--vx-border)] p-3 text-center">
                  <div className="text-xl font-bold tabular-nums">{formatTemp(status.cpu_temp_c)}</div>
                  <div className="text-[11px] text-[var(--vx-muted)]">CPU temperature</div>
                </div>
              </div>
              {status.stopped_reason && (
                <div className="rounded-lg bg-vx-500/5 border border-vx-500/20 p-3 text-xs text-[var(--vx-muted)]">
                  Stopped: {status.stopped_reason}
                </div>
              )}
            </div>
          )}
        </Card>
      </div>

      <ConfirmDialog
        open={confirmOpen} onClose={() => setConfirmOpen(false)} onConfirm={start} danger
        title="Start high-load stress test?" confirmLabel="I understand, start"
        message={`This will load the CPU heavily for up to ${formatDuration(duration)} and generate heat. Make sure cooling is adequate and stay with the device. It stops at the ${tempLimit}°C limit or when time elapses. Continue?`}
      />
    </div>
  );
}
