import { useState } from "react";
import { Network, Play, Globe } from "lucide-react";
import { useAsync } from "../hooks/useAsync";
import { getNetworkAdapters, pingHost, resolveDns, type PingResult } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { MeasurementValue } from "../components/MeasurementValue";
import { NativeRequired, EmptyState } from "../components/NativeRequired";
import { Spinner } from "../components/ProgressBar";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";

export function NetworkPage() {
  const { data, loading, refresh } = useAsync(getNetworkAdapters, []);
  const pushToast = useAppStore((s) => s.pushToast);
  const [host, setHost] = useState("1.1.1.1");
  const [pingRes, setPingRes] = useState<PingResult | null>(null);
  const [dnsHost, setDnsHost] = useState("example.com");
  const [dnsRes, setDnsRes] = useState<string[] | null>(null);
  const [busy, setBusy] = useState<string | null>(null);

  const doPing = async () => {
    if (!isNative) { pushToast({ kind: "warning", title: "Desktop app required", message: "Ping runs in the Windows desktop application." }); return; }
    setBusy("ping"); setPingRes(null);
    try { setPingRes(await pingHost(host, 4)); }
    catch (e) { pushToast({ kind: "error", title: "Ping failed", message: String(e) }); }
    finally { setBusy(null); }
  };

  const doDns = async () => {
    if (!isNative) { pushToast({ kind: "warning", title: "Desktop app required", message: "DNS resolution runs in the Windows desktop application." }); return; }
    setBusy("dns"); setDnsRes(null);
    try { setDnsRes(await resolveDns(dnsHost)); }
    catch (e) { pushToast({ kind: "error", title: "DNS resolution failed", message: String(e) }); }
    finally { setBusy(null); }
  };

  return (
    <div>
      <PageHeader title="Network Diagnostics" description="Adapters, connectivity, ping and DNS — no intrusive scanning"
        actions={<button className="vx-btn-ghost" onClick={refresh}>Refresh</button>} />
      <NativeRequired what="Network information" />

      {loading ? <div className="mt-6"><Spinner /></div> : (
        <div className="space-y-4 mt-4">
          {!data || data.length === 0 ? (
            <EmptyState title="No adapters" message="No network adapters were returned. Run the Windows desktop app to enumerate adapters." icon={<Network className="w-8 h-8" />} />
          ) : (
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
              {data.map((a, i) => (
                <Card key={i} title={a.name.value ?? `Adapter ${i + 1}`} subtitle={a.adapter_type.value ?? undefined}
                  actions={a.connected.value ? <span className="text-xs text-emerald-500 font-semibold">Connected</span> : <span className="text-xs text-[var(--vx-muted)]">Disconnected</span>}>
                  <MeasurementValue label="IP addresses" measurement={a.ip_addresses} />
                  <MeasurementValue label="MAC address" measurement={a.mac_address} />
                  <MeasurementValue label="Link speed" measurement={a.link_speed_mbps} format={(v) => `${v} Mbps`} />
                  <MeasurementValue label="DNS servers" measurement={a.dns_servers} />
                  {a.signal_pct.value != null && <MeasurementValue label="Signal strength" measurement={a.signal_pct} format={(v) => `${v}%`} />}
                </Card>
              ))}
            </div>
          )}

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <Card title="Ping test" subtitle="ICMP echo to a target you choose">
              <div className="flex gap-2 mb-3">
                <input className="vx-input" value={host} onChange={(e) => setHost(e.target.value)} placeholder="Host or IP" aria-label="Ping host" />
                <button className="vx-btn-primary" onClick={doPing} disabled={busy !== null}>
                  {busy === "ping" ? <Spinner size={16} /> : <Play className="w-4 h-4" />} Ping
                </button>
              </div>
              {pingRes && (
                <div className="grid grid-cols-2 gap-3 text-sm">
                  <Stat label="Sent / Received" value={`${pingRes.transmitted} / ${pingRes.received}`} />
                  <Stat label="Packet loss" value={`${pingRes.loss_pct.toFixed(0)}%`} />
                  <Stat label="Min / Avg / Max" value={`${pingRes.min_ms ?? "—"} / ${pingRes.avg_ms ?? "—"} / ${pingRes.max_ms ?? "—"} ms`} />
                </div>
              )}
            </Card>

            <Card title="DNS resolution test" subtitle="Resolve a hostname to addresses">
              <div className="flex gap-2 mb-3">
                <input className="vx-input" value={dnsHost} onChange={(e) => setDnsHost(e.target.value)} placeholder="Hostname" aria-label="DNS host" />
                <button className="vx-btn-primary" onClick={doDns} disabled={busy !== null}>
                  {busy === "dns" ? <Spinner size={16} /> : <Globe className="w-4 h-4" />} Resolve
                </button>
              </div>
              {dnsRes && (
                dnsRes.length === 0 ? <p className="text-sm text-[var(--vx-muted)]">No addresses returned.</p> :
                <div className="space-y-1">{dnsRes.map((ip) => <div key={ip} className="text-sm font-mono">{ip}</div>)}</div>
              )}
            </Card>
          </div>

          <p className="text-xs text-[var(--vx-muted)]">
            Network tests only contact targets you explicitly specify. VYRONIX performs no intrusive network scans.
            Personal network details are excluded from reports unless you opt in.
          </p>
        </div>
      )}
    </div>
  );
}

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-[var(--vx-border)] p-3">
      <div className="text-[11px] text-[var(--vx-muted)]">{label}</div>
      <div className="font-semibold tabular-nums mt-0.5">{value}</div>
    </div>
  );
}
