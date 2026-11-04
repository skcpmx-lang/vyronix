import { useAsync } from "../hooks/useAsync";
import { getSystemOverview } from "../lib/services";
import { Card, PageHeader } from "../components/Card";
import { MeasurementValue } from "../components/MeasurementValue";
import { NativeRequired } from "../components/NativeRequired";
import { Spinner } from "../components/ProgressBar";
import { formatDuration } from "../lib/format";

export function SystemOverview() {
  const { data, loading, refresh } = useAsync(getSystemOverview, []);

  return (
    <div>
      <PageHeader title="System Overview" description="Device, motherboard, BIOS and Windows identity — every value with its source"
        actions={<button className="vx-btn-ghost" onClick={refresh}>Refresh</button>} />
      <NativeRequired what="System information" />

      {loading ? <div className="mt-6"><Spinner /></div> : data && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4">
          <Card title="Device">
            <MeasurementValue label="Device name" measurement={data.device_name} />
            <MeasurementValue label="Manufacturer" measurement={data.manufacturer} />
            <MeasurementValue label="Model" measurement={data.model} />
            <MeasurementValue label="System type" measurement={data.system_type} />
            <MeasurementValue label="Uptime" measurement={data.uptime_secs} format={(v) => formatDuration(v)} />
            <MeasurementValue label="Boot time" measurement={data.boot_time} />
          </Card>

          <Card title="Motherboard & firmware">
            <MeasurementValue label="Motherboard manufacturer" measurement={data.motherboard_manufacturer} />
            <MeasurementValue label="Motherboard model" measurement={data.motherboard_model} />
            <MeasurementValue label="BIOS version" measurement={data.bios_version} />
            <MeasurementValue label="BIOS date" measurement={data.bios_date} />
          </Card>

          <Card title="Windows">
            <MeasurementValue label="Edition" measurement={data.os_edition} />
            <MeasurementValue label="Version" measurement={data.os_version} />
            <MeasurementValue label="Build" measurement={data.os_build} />
            <MeasurementValue label="Architecture" measurement={data.os_arch} />
            <MeasurementValue label="Install date" measurement={data.install_date} />
          </Card>

          <Card title="Security & platform">
            <MeasurementValue label="Secure Boot" measurement={data.secure_boot} />
            <MeasurementValue label="TPM" measurement={data.tpm} />
            <MeasurementValue label="Virtualization" measurement={data.virtualization} />
          </Card>
        </div>
      )}
    </div>
  );
}
