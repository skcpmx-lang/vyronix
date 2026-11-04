import { useState } from "react";
import { useAsync } from "../hooks/useAsync";
import { getMemoryInfo, runMemoryTest } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { MeasurementValue } from "../components/MeasurementValue";
import { NativeRequired, EmptyState } from "../components/NativeRequired";
import { Spinner, ProgressBar } from "../components/ProgressBar";
import { ConfirmDialog } from "../components/Modal";
import { formatBytes, formatPct } from "../lib/format";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";
import type { MemoryTestResult } from "../lib/types";
import { MemoryStick, FlaskConical } from "lucide-react";

export function MemoryPage() {
  const { data, loading, refresh } = useAsync(getMemoryInfo, []);
  const pushToast = useAppStore((s) => s.pushToast);
  const [testing, setTesting] = useState(false);
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [result, setResult] = useState<MemoryTestResult | null>(null);

  const startTest = async () => {
    if (!isNative) {
      pushToast({ kind: "warning", title: "Desktop app required", message: "Memory testing runs in the Windows desktop application." });
      return;
    }
    setTesting(true);
    setResult(null);
    try {
      const r = await runMemoryTest(256);
      setResult(r);
      pushToast({
        kind: r.passed ? "success" : "error",
        title: r.passed ? "Memory test passed" : "Memory errors detected",
        message: `${r.errors_found} error(s) across ${r.patterns_tested.length} patterns.`,
      });
    } catch (e) {
      pushToast({ kind: "error", title: "Memory test failed to run", message: e instanceof Error ? e.message : String(e) });
    } finally {
      setTesting(false);
    }
  };

  return (
    <div>
      <PageHeader title="Memory" description="RAM capacity, modules and application-level memory testing"
        actions={<button className="vx-btn-ghost" onClick={refresh}>Refresh</button>} />
      <NativeRequired what="Memory information" />

      {loading ? <div className="mt-6"><Spinner /></div> : data && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4">
          <Card title="Capacity & usage">
            <MeasurementValue label="Installed" measurement={data.total_bytes} format={formatBytes} />
            <MeasurementValue label="Used" measurement={data.used_bytes} format={formatBytes} />
            <MeasurementValue label="Available" measurement={data.available_bytes} format={formatBytes} />
            <MeasurementValue label="Usage" measurement={data.usage_pct} format={formatPct} />
            {data.usage_pct.value != null && (
              <ProgressBar value={data.usage_pct.value} className="mt-2"
                color={data.usage_pct.value > 90 ? "#ef4444" : "#3391ff"} />
            )}
          </Card>

          <Card title="Memory modules" subtitle="Per-DIMM details from SMBIOS / WMI">
            {data.modules.length === 0 ? (
              <EmptyState title="No module details" message="Module information was not exposed by the hardware interface." icon={<MemoryStick className="w-7 h-7" />} />
            ) : (
              <div className="space-y-3">
                {data.modules.map((m, i) => (
                  <div key={i} className="rounded-lg border border-[var(--vx-border)] p-3">
                    <div className="text-sm font-semibold mb-1">{m.slot.value ?? `Slot ${i + 1}`}</div>
                    <div className="grid grid-cols-2 gap-x-4 text-xs">
                      <div className="text-[var(--vx-muted)]">Capacity: <span className="text-[var(--vx-text)]">{formatBytes(m.capacity_bytes.value)}</span></div>
                      <div className="text-[var(--vx-muted)]">Speed: <span className="text-[var(--vx-text)]">{m.speed_mhz.value != null ? `${m.speed_mhz.value} MHz` : "—"}</span></div>
                      <div className="text-[var(--vx-muted)]">Type: <span className="text-[var(--vx-text)]">{m.memory_type.value ?? "—"}</span></div>
                      <div className="text-[var(--vx-muted)]">Maker: <span className="text-[var(--vx-text)]">{m.manufacturer.value ?? "—"}</span></div>
                    </div>
                    {m.part_number.value && <div className="text-[11px] text-[var(--vx-muted)] mt-1 font-mono">{m.part_number.value}</div>}
                  </div>
                ))}
              </div>
            )}
          </Card>

          <Card title="Memory test" subtitle="Application-level read/write pattern verification" className="lg:col-span-2"
            actions={
              <button className="vx-btn-primary" onClick={() => setConfirmOpen(true)} disabled={testing}>
                {testing ? <Spinner size={16} /> : <FlaskConical className="w-4 h-4" />}
                {testing ? "Testing…" : "Run memory test"}
              </button>
            }>
            <p className="text-xs text-[var(--vx-muted)] leading-relaxed mb-3">
              This test allocates a buffer in this process and verifies multiple write/read patterns
              (zeros, ones, alternating, walking-ones, address-based). It detects memory errors reachable
              by a normal Windows process. It is <strong>not</strong> a comprehensive physical RAM test —
              that requires a bootable diagnostic environment, which VYRONIX does not perform automatically.
            </p>
            {result && (
              <div className={`rounded-lg border p-4 ${result.passed ? "border-emerald-500/30 bg-emerald-500/5" : "border-red-500/30 bg-red-500/5"}`}>
                <div className="text-sm font-semibold mb-2">{result.passed ? "Passed — no errors detected" : `Failed — ${result.errors_found} error(s)`}</div>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-3 text-xs">
                  <div><div className="text-[var(--vx-muted)]">Bytes tested</div><div className="font-semibold">{formatBytes(result.bytes_tested)}</div></div>
                  <div><div className="text-[var(--vx-muted)]">Patterns</div><div className="font-semibold">{result.patterns_tested.length}</div></div>
                  <div><div className="text-[var(--vx-muted)]">Errors</div><div className="font-semibold">{result.errors_found}</div></div>
                  <div><div className="text-[var(--vx-muted)]">Duration</div><div className="font-semibold">{(result.duration_ms / 1000).toFixed(1)}s</div></div>
                </div>
                <div className="text-[11px] text-[var(--vx-muted)] mt-2">{result.note}</div>
              </div>
            )}
          </Card>
        </div>
      )}

      <ConfirmDialog
        open={confirmOpen} onClose={() => setConfirmOpen(false)} onConfirm={startTest}
        title="Run memory test?" confirmLabel="Start test"
        message="This allocates ~256 MB and runs read/write verification patterns. It is safe and non-destructive, but may briefly increase memory pressure. Continue?"
      />
    </div>
  );
}
