import { useAsync } from "../hooks/useAsync";
import { getGpuInfo } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { MeasurementValue } from "../components/MeasurementValue";
import { NativeRequired, EmptyState } from "../components/NativeRequired";
import { Spinner } from "../components/ProgressBar";
import { formatBytes, formatPct, formatTemp } from "../lib/format";
import { CircuitBoard } from "lucide-react";

export function GpuPage() {
  const { data, loading, refresh } = useAsync(getGpuInfo, []);

  return (
    <div>
      <PageHeader title="Graphics" description="Integrated and dedicated GPU adapters, drivers and memory"
        actions={<button className="vx-btn-ghost" onClick={refresh}>Refresh</button>} />
      <NativeRequired what="GPU information" />

      {loading ? <div className="mt-6"><Spinner /></div> : (
        !data || data.length === 0 ? (
          <div className="mt-4"><EmptyState title="No GPU data" message="No display adapters were returned. Run the Windows desktop app to enumerate GPUs." icon={<CircuitBoard className="w-8 h-8" />} /></div>
        ) : (
          <div className="space-y-4 mt-4">
            {data.map((gpu, i) => (
              <Card key={i} title={gpu.name.value ?? `GPU ${i + 1}`} subtitle={gpu.manufacturer.value ?? undefined}>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-x-8">
                  <div>
                    <MeasurementValue label="Driver version" measurement={gpu.driver_version} />
                    <MeasurementValue label="Driver date" measurement={gpu.driver_date} />
                    <MeasurementValue label="Dedicated memory" measurement={gpu.dedicated_memory_bytes} format={formatBytes} />
                    <MeasurementValue label="Shared memory" measurement={gpu.shared_memory_bytes} format={formatBytes} />
                  </div>
                  <div>
                    <MeasurementValue label="Current resolution" measurement={gpu.current_resolution} />
                    <MeasurementValue label="Refresh rate" measurement={gpu.refresh_rate_hz} format={(v) => `${v} Hz`} />
                    <MeasurementValue label="Utilization" measurement={gpu.usage_pct} format={formatPct} />
                    <MeasurementValue label="Temperature" measurement={gpu.temperature_c} format={formatTemp} />
                  </div>
                </div>
              </Card>
            ))}
          </div>
        )
      )}
    </div>
  );
}
