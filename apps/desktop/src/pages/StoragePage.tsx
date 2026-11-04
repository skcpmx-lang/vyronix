import { HardDrive } from "lucide-react";
import { useAsync } from "../hooks/useAsync";
import { getStorageDevices } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { MeasurementValue } from "../components/MeasurementValue";
import { SeverityBadge } from "../components/StatusBadge";
import { NativeRequired, EmptyState } from "../components/NativeRequired";
import { Spinner, ProgressBar } from "../components/ProgressBar";
import { formatBytes, formatTemp, formatNumber } from "../lib/format";
import { useAppStore } from "../lib/store";

export function StoragePage() {
  const { data, loading, refresh } = useAsync(getStorageDevices, []);
  const setPage = useAppStore((s) => s.setPage);

  return (
    <div>
      <PageHeader title="Storage Health" description="HDD / SATA SSD / NVMe SMART diagnostics and partitions"
        actions={
          <>
            <button className="vx-btn-ghost" onClick={() => setPage("benchmarks")}>Storage benchmark</button>
            <button className="vx-btn-ghost" onClick={refresh}>Refresh</button>
          </>
        } />
      <NativeRequired what="Storage information" />

      {loading ? <div className="mt-6"><Spinner /></div> : (
        !data || data.length === 0 ? (
          <div className="mt-4"><EmptyState title="No storage devices" message="No drives were returned. Run the Windows desktop app to enumerate storage." icon={<HardDrive className="w-8 h-8" />} /></div>
        ) : (
          <div className="space-y-4 mt-4">
            {data.map((dev) => (
              <Card key={dev.id} title={dev.model.value ?? "Unknown drive"}
                subtitle={[dev.media_type.value, dev.interface.value].filter(Boolean).join(" · ") || undefined}
                actions={<SeverityBadge severity={dev.severity} />}>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-x-8">
                  <div>
                    <MeasurementValue label="Capacity" measurement={dev.capacity_bytes} format={formatBytes} />
                    <MeasurementValue label="Firmware" measurement={dev.firmware} />
                    <MeasurementValue label="Health status" measurement={dev.health_status} />
                    <MeasurementValue label="Temperature" measurement={dev.temperature_c} format={formatTemp} />
                  </div>
                  <div>
                    <MeasurementValue label="Power-on hours" measurement={dev.power_on_hours} format={formatNumber} />
                    <MeasurementValue label="Power cycles" measurement={dev.power_cycles} format={formatNumber} />
                    <MeasurementValue label="Wear / used" measurement={dev.percentage_used} format={(v) => `${v}%`} />
                    <MeasurementValue label="Available spare" measurement={dev.available_spare_pct} format={(v) => `${v}%`} />
                  </div>
                  <div>
                    <MeasurementValue label="Media errors" measurement={dev.media_errors} format={formatNumber} />
                    <MeasurementValue label="Reallocated sectors" measurement={dev.reallocated_sectors} format={formatNumber} />
                    <MeasurementValue label="Pending sectors" measurement={dev.pending_sectors} format={formatNumber} />
                    <MeasurementValue label="Uncorrectable sectors" measurement={dev.uncorrectable_sectors} format={formatNumber} />
                  </div>
                </div>

                {dev.smart_notes.length > 0 && (
                  <div className="mt-3 rounded-lg bg-amber-500/5 border border-amber-500/20 p-3">
                    <div className="text-xs font-semibold text-amber-500 mb-1">SMART notes</div>
                    <ul className="text-xs text-[var(--vx-muted)] space-y-0.5 list-disc list-inside">
                      {dev.smart_notes.map((n, i) => <li key={i}>{n}</li>)}
                    </ul>
                  </div>
                )}

                {dev.partitions.length > 0 && (
                  <div className="mt-4">
                    <div className="vx-label mb-2">Partitions</div>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                      {dev.partitions.map((p, i) => {
                        const usedPct = p.capacity_bytes.value && p.free_bytes.value != null
                          ? ((p.capacity_bytes.value - p.free_bytes.value) / p.capacity_bytes.value) * 100 : null;
                        return (
                          <div key={i} className="rounded-lg border border-[var(--vx-border)] p-3">
                            <div className="flex items-center justify-between text-sm font-medium mb-1">
                              <span>{p.drive_letter || "Partition"} {p.label.value ? `· ${p.label.value}` : ""}</span>
                              <span className="text-xs text-[var(--vx-muted)]">{p.file_system.value ?? "—"}</span>
                            </div>
                            {usedPct != null && <ProgressBar value={usedPct} color={usedPct > 90 ? "#ef4444" : "#3391ff"} />}
                            <div className="text-[11px] text-[var(--vx-muted)] mt-1">
                              {formatBytes(p.free_bytes.value)} free of {formatBytes(p.capacity_bytes.value)}
                            </div>
                          </div>
                        );
                      })}
                    </div>
                  </div>
                )}
              </Card>
            ))}
            <p className="text-xs text-[var(--vx-muted)]">
              SMART interpretation uses vendor thresholds and NVMe spec fields — never a single universal cutoff.
              Unknown health is reported as unknown, not good. Administrator access may be required for ATA SMART on some systems.
            </p>
          </div>
        )
      )}
    </div>
  );
}
