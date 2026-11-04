import { useState } from "react";
import { Cpu, MemoryStick, HardDrive, Play, History } from "lucide-react";
import { Card, PageHeader } from "../components/Card";
import { ConfirmDialog } from "../components/Modal";
import { Spinner } from "../components/ProgressBar";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";
import { runCpuBenchmark, runMemoryBenchmark, runStorageBenchmark } from "../lib/services";
import type { CpuBenchResult, MemoryBenchResult, StorageBenchResult } from "../lib/types";
import { formatBytes, formatNumber } from "../lib/format";

type HistEntry = { kind: string; when: number; summary: string };

export function Benchmarks() {
  const pushToast = useAppStore((s) => s.pushToast);
  const [cpuDur, setCpuDur] = useState(5);
  const [memSize, setMemSize] = useState(256);
  const [stoSize, setStoSize] = useState(256);
  const [stoPath, setStoPath] = useState("");
  const [busy, setBusy] = useState<string | null>(null);
  const [cpuRes, setCpuRes] = useState<CpuBenchResult | null>(null);
  const [memRes, setMemRes] = useState<MemoryBenchResult | null>(null);
  const [stoRes, setStoRes] = useState<StorageBenchResult | null>(null);
  const [stoConfirm, setStoConfirm] = useState(false);
  const [history, setHistory] = useState<HistEntry[]>([]);

  const requireNative = () => {
    if (!isNative) {
      pushToast({ kind: "warning", title: "Desktop app required", message: "Benchmarks run real workloads in the Windows desktop application." });
      return false;
    }
    return true;
  };

  const addHistory = (kind: string, summary: string) =>
    setHistory((h) => [{ kind, when: Date.now(), summary }, ...h].slice(0, 20));

  const doCpu = async () => {
    if (!requireNative()) return;
    setBusy("cpu");
    try {
      const r = await runCpuBenchmark(cpuDur);
      setCpuRes(r);
      addHistory("CPU", `Single ${formatNumber(Math.round(r.single_core_score))} · Multi ${formatNumber(Math.round(r.multi_core_score))}`);
    } catch (e) { pushToast({ kind: "error", title: "CPU benchmark failed", message: String(e) }); }
    finally { setBusy(null); }
  };

  const doMem = async () => {
    if (!requireNative()) return;
    setBusy("mem");
    try {
      const r = await runMemoryBenchmark(memSize);
      setMemRes(r);
      addHistory("Memory", `Read ${r.read_bandwidth_mbps.toFixed(0)} · Write ${r.write_bandwidth_mbps.toFixed(0)} MB/s`);
    } catch (e) { pushToast({ kind: "error", title: "Memory benchmark failed", message: String(e) }); }
    finally { setBusy(null); }
  };

  const doSto = async () => {
    if (!requireNative()) return;
    if (!stoPath) { pushToast({ kind: "warning", title: "Choose a test folder", message: "Enter a folder path on the drive you want to benchmark." }); return; }
    setBusy("sto");
    try {
      const r = await runStorageBenchmark(stoPath, stoSize);
      setStoRes(r);
      addHistory("Storage", `Seq R ${r.sequential_read_mbps.toFixed(0)} / W ${r.sequential_write_mbps.toFixed(0)} MB/s`);
    } catch (e) { pushToast({ kind: "error", title: "Storage benchmark failed", message: String(e) }); }
    finally { setBusy(null); }
  };

  return (
    <div>
      <PageHeader title="Benchmarks" description="Real, reproducible CPU, memory and storage performance tests" />

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* CPU */}
        <Card title="CPU benchmark" subtitle="Integer & floating-point workloads">
          <div className="flex items-center gap-2 mb-3">
            <label className="text-xs text-[var(--vx-muted)]">Duration</label>
            <select className="vx-input w-auto" value={cpuDur} onChange={(e) => setCpuDur(Number(e.target.value))}>
              {[3, 5, 10, 20].map((d) => <option key={d} value={d}>{d}s</option>)}
            </select>
            <button className="vx-btn-primary ml-auto" onClick={doCpu} disabled={busy !== null}>
              {busy === "cpu" ? <Spinner size={16} /> : <Play className="w-4 h-4" />} Run
            </button>
          </div>
          {cpuRes ? (
            <div className="space-y-2 text-sm">
              <Row label="Single-core score" value={formatNumber(Math.round(cpuRes.single_core_score))} />
              <Row label="Multi-core score" value={formatNumber(Math.round(cpuRes.multi_core_score))} />
              <Row label="Integer ops/s" value={formatNumber(Math.round(cpuRes.integer_ops_per_sec))} />
              <Row label="Float ops/s" value={formatNumber(Math.round(cpuRes.float_ops_per_sec))} />
              <Row label="Threads" value={String(cpuRes.threads)} />
              <p className="text-[11px] text-[var(--vx-muted)]">Scores are relative to this build's workload; compare like-for-like.</p>
            </div>
          ) : <Placeholder icon={<Cpu className="w-6 h-6" />} text="Run to measure CPU performance" />}
        </Card>

        {/* Memory */}
        <Card title="Memory benchmark" subtitle="Read / write / copy bandwidth">
          <div className="flex items-center gap-2 mb-3">
            <label className="text-xs text-[var(--vx-muted)]">Size</label>
            <select className="vx-input w-auto" value={memSize} onChange={(e) => setMemSize(Number(e.target.value))}>
              {[128, 256, 512].map((d) => <option key={d} value={d}>{d} MiB</option>)}
            </select>
            <button className="vx-btn-primary ml-auto" onClick={doMem} disabled={busy !== null}>
              {busy === "mem" ? <Spinner size={16} /> : <Play className="w-4 h-4" />} Run
            </button>
          </div>
          {memRes ? (
            <div className="space-y-2 text-sm">
              <Row label="Read bandwidth" value={`${memRes.read_bandwidth_mbps.toFixed(0)} MB/s`} />
              <Row label="Write bandwidth" value={`${memRes.write_bandwidth_mbps.toFixed(0)} MB/s`} />
              <Row label="Copy bandwidth" value={`${memRes.copy_bandwidth_mbps.toFixed(0)} MB/s`} />
              <Row label="Test size" value={formatBytes(memRes.test_size_bytes)} />
            </div>
          ) : <Placeholder icon={<MemoryStick className="w-6 h-6" />} text="Run to measure memory bandwidth" />}
        </Card>

        {/* Storage */}
        <Card title="Storage benchmark" subtitle="Sequential & random I/O on a temp file">
          <div className="space-y-2 mb-3">
            <input className="vx-input" placeholder="Test folder, e.g. C:\VXTest" value={stoPath}
              onChange={(e) => setStoPath(e.target.value)} aria-label="Storage test folder" />
            <div className="flex items-center gap-2">
              <select className="vx-input w-auto" value={stoSize} onChange={(e) => setStoSize(Number(e.target.value))}>
                {[64, 128, 256, 512].map((d) => <option key={d} value={d}>{d} MiB</option>)}
              </select>
              <button className="vx-btn-primary ml-auto" onClick={() => setStoConfirm(true)} disabled={busy !== null}>
                {busy === "sto" ? <Spinner size={16} /> : <Play className="w-4 h-4" />} Run
              </button>
            </div>
          </div>
          {stoRes ? (
            <div className="space-y-2 text-sm">
              <Row label="Sequential read" value={`${stoRes.sequential_read_mbps.toFixed(0)} MB/s`} />
              <Row label="Sequential write" value={`${stoRes.sequential_write_mbps.toFixed(0)} MB/s`} />
              <Row label="Random read IOPS" value={formatNumber(Math.round(stoRes.random_read_iops))} />
              <Row label="Random write IOPS" value={formatNumber(Math.round(stoRes.random_write_iops))} />
              <Row label="Read latency" value={`${stoRes.random_read_latency_ms.toFixed(2)} ms`} />
              <Row label="Write latency" value={`${stoRes.random_write_latency_ms.toFixed(2)} ms`} />
            </div>
          ) : <Placeholder icon={<HardDrive className="w-6 h-6" />} text="Run to measure storage performance" />}
        </Card>
      </div>

      <Card title="Recent results" subtitle="This session" className="mt-4">
        {history.length === 0 ? (
          <p className="text-sm text-[var(--vx-muted)] flex items-center gap-2"><History className="w-4 h-4" /> No benchmarks run yet.</p>
        ) : (
          <div className="space-y-1">
            {history.map((h, i) => (
              <div key={i} className="flex items-center justify-between text-sm py-1.5 border-b border-[var(--vx-border)] last:border-0">
                <span className="font-medium">{h.kind}</span>
                <span className="text-[var(--vx-muted)] text-xs flex-1 mx-3 truncate">{h.summary}</span>
                <span className="text-[11px] text-[var(--vx-muted)]">{new Date(h.when).toLocaleTimeString()}</span>
              </div>
            ))}
          </div>
        )}
      </Card>

      <ConfirmDialog
        open={stoConfirm} onClose={() => setStoConfirm(false)} onConfirm={doSto}
        title="Run storage benchmark?" confirmLabel="Start benchmark"
        message="This writes a dedicated temporary test file in the chosen folder, measures read/write performance, then deletes it. It never touches your existing files or raw device sectors. Free disk space is checked first. Ensure the folder is on the drive you want to test. Continue?"
      />
    </div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-xs text-[var(--vx-muted)]">{label}</span>
      <span className="font-semibold tabular-nums">{value}</span>
    </div>
  );
}

function Placeholder({ icon, text }: { icon: React.ReactNode; text: string }) {
  return (
    <div className="flex flex-col items-center justify-center py-6 text-[var(--vx-muted)]">
      {icon}
      <span className="text-xs mt-2">{text}</span>
    </div>
  );
}
