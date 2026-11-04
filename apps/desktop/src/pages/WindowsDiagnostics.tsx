import { useState } from "react";
import { useAsync } from "../hooks/useAsync";
import { getWindowsDiagnostics } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { NativeRequired, EmptyState } from "../components/NativeRequired";
import { Spinner } from "../components/ProgressBar";
import { formatBytes } from "../lib/format";
import { AppWindow } from "lucide-react";

type Tab = "processes" | "services" | "drivers" | "startup" | "events";

export function WindowsDiagnostics() {
  const { data, loading, refresh } = useAsync(getWindowsDiagnostics, []);
  const [tab, setTab] = useState<Tab>("processes");
  const [filter, setFilter] = useState("");

  const tabs: { id: Tab; label: string; count: number }[] = [
    { id: "processes", label: "Processes", count: data?.processes.length ?? 0 },
    { id: "services", label: "Services", count: data?.services.length ?? 0 },
    { id: "drivers", label: "Drivers", count: data?.drivers.length ?? 0 },
    { id: "startup", label: "Startup", count: data?.startup.length ?? 0 },
    { id: "events", label: "Critical events", count: data?.critical_events.length ?? 0 },
  ];

  const f = filter.toLowerCase();

  return (
    <div>
      <PageHeader title="Windows Diagnostics" description="Read-only view of processes, services, drivers, startup and critical events"
        actions={<button className="vx-btn-ghost" onClick={refresh}>Refresh</button>} />
      <NativeRequired what="Windows diagnostics" />
      <p className="text-xs text-[var(--vx-muted)] mb-4">
        VYRONIX only reads this information. It never modifies system settings, registry values, drivers or Windows
        security features automatically.
      </p>

      {loading ? <div className="mt-6"><Spinner /></div> : data && (
        <Card>
          <div className="flex flex-wrap items-center gap-2 mb-4">
            {tabs.map((t) => (
              <button key={t.id} onClick={() => setTab(t.id)}
                className={`px-3 py-1.5 rounded-lg text-sm transition ${tab === t.id ? "bg-vx-600/15 text-vx-500 font-semibold border border-vx-600/30" : "hover:bg-vx-500/10 border border-transparent"}`}>
                {t.label} <span className="text-xs text-[var(--vx-muted)]">({t.count})</span>
              </button>
            ))}
            <input className="vx-input w-auto ml-auto" placeholder="Filter…" value={filter} onChange={(e) => setFilter(e.target.value)} aria-label="Filter" />
          </div>

          <div className="overflow-x-auto max-h-[480px] overflow-y-auto">
            {tab === "processes" && (
              <Table headers={["PID", "Name", "CPU %", "Memory"]}
                rows={data.processes.filter((p) => p.name.toLowerCase().includes(f)).slice(0, 300)
                  .map((p) => [String(p.pid), p.name, p.cpu_pct.toFixed(1), formatBytes(p.memory_bytes)])} />
            )}
            {tab === "services" && (
              <Table headers={["Name", "Display name", "State", "Start mode"]}
                rows={data.services.filter((s) => (s.name + s.display_name).toLowerCase().includes(f)).slice(0, 300)
                  .map((s) => [s.name, s.display_name, s.state, s.start_mode])} />
            )}
            {tab === "drivers" && (
              <Table headers={["Device", "Version", "Date", "Status"]}
                rows={data.drivers.filter((d) => d.name.toLowerCase().includes(f)).slice(0, 300)
                  .map((d) => [d.name, d.version, d.date, d.error_code !== 0 ? `Problem (code ${d.error_code})` : d.status])} />
            )}
            {tab === "startup" && (
              <Table headers={["Name", "Command", "Location"]}
                rows={data.startup.filter((s) => s.name.toLowerCase().includes(f))
                  .map((s) => [s.name, s.command, s.location])} />
            )}
            {tab === "events" && (
              data.critical_events.length === 0 ? (
                <EmptyState title="No critical events" message="No critical events were found in the recent log window, or the event log is not accessible." icon={<AppWindow className="w-7 h-7" />} />
              ) : (
                <div className="space-y-2">
                  {data.critical_events.filter((e) => (e.source + e.message).toLowerCase().includes(f)).slice(0, 100).map((e, i) => (
                    <div key={i} className="rounded-lg border border-red-500/20 bg-red-500/5 p-3">
                      <div className="flex items-center justify-between text-xs">
                        <span className="font-semibold text-red-400">{e.source} · Event {e.event_id}</span>
                        <span className="text-[var(--vx-muted)]">{e.time}</span>
                      </div>
                      <div className="text-xs text-[var(--vx-muted)] mt-1 line-clamp-3">{e.message}</div>
                    </div>
                  ))}
                </div>
              )
            )}
          </div>
        </Card>
      )}
    </div>
  );
}

function Table({ headers, rows }: { headers: string[]; rows: string[][] }) {
  if (rows.length === 0) return <p className="text-sm text-[var(--vx-muted)] py-4">No matching entries.</p>;
  return (
    <table className="w-full text-sm">
      <thead className="sticky top-0 bg-[var(--vx-panel)]">
        <tr className="text-left text-xs text-[var(--vx-muted)] border-b border-[var(--vx-border)]">
          {headers.map((h) => <th key={h} className="py-2 pr-4 font-semibold">{h}</th>)}
        </tr>
      </thead>
      <tbody>
        {rows.map((r, i) => (
          <tr key={i} className="border-b border-[var(--vx-border)] last:border-0">
            {r.map((c, j) => <td key={j} className="py-1.5 pr-4 text-xs max-w-[280px] truncate" title={c}>{c}</td>)}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
