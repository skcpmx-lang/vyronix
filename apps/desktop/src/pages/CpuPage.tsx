import { useAsync, usePoll } from "../hooks/useAsync";
import { getCpuInfo, getMonitoringSample } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { MeasurementValue } from "../components/MeasurementValue";
import { LiveChart } from "../components/LiveChart";
import { NativeRequired } from "../components/NativeRequired";
import { Spinner } from "../components/ProgressBar";
import { formatFreq, formatPct, formatTemp } from "../lib/format";
import { isNative } from "../lib/ipc";
import { useAppStore } from "../lib/store";
import { useEffect, useState } from "react";

export function CpuPage() {
  const { data, loading, refresh } = useAsync(getCpuInfo, []);
  const interval = useAppStore((s) => s.settings.monitoring_interval_ms);
  const sample = usePoll(getMonitoringSample, interval, isNative);
  const [usageHistory, setUsageHistory] = useState<(number | null)[]>([]);

  useEffect(() => {
    if (sample?.cpu_usage_pct != null) {
      setUsageHistory((h) => [...h.slice(-59), sample.cpu_usage_pct]);
    }
  }, [sample]);

  return (
    <div>
      <PageHeader title="Processor" description="CPU specifications, live usage and temperature"
        actions={<button className="vx-btn-ghost" onClick={refresh}>Refresh</button>} />
      <NativeRequired what="CPU information" />

      {loading ? <div className="mt-6"><Spinner /></div> : data && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4">
          <Card title="Identification">
            <MeasurementValue label="Model" measurement={data.name} />
            <MeasurementValue label="Manufacturer" measurement={data.manufacturer} />
            <MeasurementValue label="Architecture" measurement={data.architecture} />
            <MeasurementValue label="Physical cores" measurement={data.physical_cores} format={(v) => String(v)} />
            <MeasurementValue label="Logical processors" measurement={data.logical_processors} format={(v) => String(v)} />
            <MeasurementValue label="Virtualization" measurement={data.virtualization} />
          </Card>

          <Card title="Frequency & cache">
            <MeasurementValue label="Base frequency" measurement={data.base_frequency_mhz} format={formatFreq} />
            <MeasurementValue label="Current frequency" measurement={data.current_frequency_mhz} format={formatFreq} />
            <MeasurementValue label="Maximum frequency" measurement={data.max_frequency_mhz} format={formatFreq} />
            <MeasurementValue label="L2 cache" measurement={data.l2_cache_kb} format={(v) => `${v} KB`} />
            <MeasurementValue label="L3 cache" measurement={data.l3_cache_kb} format={(v) => `${v} KB`} />
          </Card>

          <Card title="Live usage" subtitle={`Polling every ${interval} ms`}>
            <div className="text-3xl font-bold tabular-nums mb-3">{formatPct(sample?.cpu_usage_pct ?? data.usage_pct.value)}</div>
            <LiveChart data={usageHistory} max={100} unit="%" color="#3391ff" />
          </Card>

          <Card title="Thermal">
            <MeasurementValue label="Temperature" measurement={data.temperature_c} format={formatTemp} />
            <p className="text-xs text-[var(--vx-muted)] mt-2 leading-relaxed">
              CPU temperature is only shown when a supported sensor interface exposes it. VYRONIX never
              infers temperature from CPU usage. If unavailable, your hardware may require vendor-specific
              software or administrator access.
            </p>
          </Card>
        </div>
      )}
    </div>
  );
}
