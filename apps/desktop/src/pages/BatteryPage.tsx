import { BatteryCharging, Plug, Zap } from "lucide-react";
import { useAsync } from "../hooks/useAsync";
import { getBatteryInfo } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { MeasurementValue } from "../components/MeasurementValue";
import { Gauge } from "../components/Gauge";
import { NativeRequired, EmptyState } from "../components/NativeRequired";
import { Spinner } from "../components/ProgressBar";
import { StatusBadge } from "../components/StatusBadge";
import { formatMwh, formatPct, formatHours, formatTemp } from "../lib/format";

export function BatteryPage() {
  const { data, loading, refresh } = useAsync(getBatteryInfo, []);

  return (
    <div>
      <PageHeader title="Battery Diagnostics" description="Health, wear, charging and capacity analysis from real battery interfaces"
        actions={<button className="vx-btn-ghost" onClick={refresh}>Refresh</button>} />
      <NativeRequired what="Battery information" />

      {loading ? <div className="mt-6"><Spinner /></div> : data && !data.present ? (
        <div className="mt-4">
          <EmptyState title="No battery detected" message="This device appears to be a desktop or has no battery exposed to Windows. Battery diagnostics are unavailable." icon={<BatteryCharging className="w-8 h-8" />} />
        </div>
      ) : data && (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 mt-4">
          {/* Health gauge */}
          <Card className="flex flex-col items-center justify-center text-center">
            <Gauge value={data.health_pct.value} label="Health" size={150} />
            <div className="mt-3"><StatusBadge status={data.health_pct.status} /></div>
            <div className="text-xs text-[var(--vx-muted)] mt-2">Wear: {formatPct(data.wear_pct.value)}</div>
            <p className="text-[11px] text-[var(--vx-muted)] mt-3 leading-relaxed">
              Health = full-charge capacity ÷ design capacity × 100. Computed only when both values are
              valid; never clamped or fabricated.
            </p>
          </Card>

          {/* Status */}
          <Card title="Current status">
            <MeasurementValue label="Charging status" measurement={data.charging_status} />
            <MeasurementValue label="AC connected" measurement={data.ac_connected} format={(v) => (v ? "Yes" : "No")} />
            <MeasurementValue label="State of charge" measurement={data.state_of_charge_pct} format={formatPct} />
            <MeasurementValue label="Charge rate" measurement={data.charge_rate_mw} format={(v) => `${(v / 1000).toFixed(1)} W`} />
            <MeasurementValue label="Discharge rate" measurement={data.discharge_rate_mw} format={(v) => `${(v / 1000).toFixed(1)} W`} />
            <MeasurementValue label="Est. runtime" measurement={data.estimated_runtime_hours} format={formatHours} />
          </Card>

          {/* Identity */}
          <Card title="Battery identity">
            <MeasurementValue label="Manufacturer" measurement={data.manufacturer} />
            <MeasurementValue label="Model" measurement={data.model} />
            <MeasurementValue label="Chemistry" measurement={data.chemistry} />
            <MeasurementValue label="Cycle count" measurement={data.cycle_count} format={(v) => String(v)} />
            <MeasurementValue label="Temperature" measurement={data.temperature_c} format={formatTemp} />
          </Card>

          {/* Capacities */}
          <Card title="Capacity analysis" className="lg:col-span-2">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8">
              <div>
                <MeasurementValue label="Design capacity" measurement={data.design_capacity_mwh} format={formatMwh} />
                <MeasurementValue label="Full charge capacity" measurement={data.full_charge_capacity_mwh} format={formatMwh} />
                <MeasurementValue label="Remaining capacity" measurement={data.remaining_capacity_mwh} format={formatMwh} />
              </div>
              <div>
                <MeasurementValue label="Design voltage" measurement={data.design_voltage_mv} format={(v) => `${(v / 1000).toFixed(2)} V`} />
                <MeasurementValue label="Current voltage" measurement={data.current_voltage_mv} format={(v) => `${(v / 1000).toFixed(2)} V`} />
              </div>
            </div>
            {data.health_pct.value != null && data.design_capacity_mwh.value != null && data.full_charge_capacity_mwh.value != null && (
              <div className="mt-3 rounded-lg bg-vx-500/5 border border-vx-500/20 p-3 text-xs text-[var(--vx-muted)]">
                <Zap className="w-3.5 h-3.5 inline mr-1 text-vx-500" />
                Degradation: {formatMwh(data.design_capacity_mwh.value - data.full_charge_capacity_mwh.value)} lost
                relative to design capacity.
              </div>
            )}
          </Card>

          {/* Methodology */}
          <Card title="Methodology & honesty">
            <div className="space-y-2 text-xs text-[var(--vx-muted)] leading-relaxed">
              <p className="flex gap-2"><Plug className="w-3.5 h-3.5 shrink-0 mt-0.5 text-vx-500" />
                Capacities come from the Windows battery IOCTL / WMI interfaces. Cycle count is shown only
                when the firmware reports it — never invented.</p>
              <p>Estimated runtime is derived from remaining capacity ÷ current discharge rate and is
                labelled as an estimate. Suspicious readings are flagged as anomalies, not silently clamped.</p>
              <p>Historical battery records are stored locally only when collected over time; no history is fabricated.</p>
            </div>
          </Card>
        </div>
      )}
    </div>
  );
}
