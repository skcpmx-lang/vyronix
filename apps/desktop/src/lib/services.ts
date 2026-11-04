// Hardware service layer. Each function calls the native backend; in a browser
// preview it returns an honest "unavailable" snapshot (never mock data).

import { invoke, isNative, unavailable, NativeUnavailableError } from "./ipc";
import type {
  SystemOverview, CpuInfo, GpuInfo, MemoryInfo, BatteryInfo, StorageDevice,
  NetworkAdapter, MonitoringSample, DiagnosticReport, CpuBenchResult,
  MemoryBenchResult, StorageBenchResult, MemoryTestResult, StressStatus,
  InspectionItem, AppSettings, AppInfo,
} from "./types";
import { DEFAULT_SETTINGS } from "./types";

const SRC = "native backend";

function unavailOverview(): SystemOverview {
  const u = <T,>() => unavailable<T>(SRC);
  return {
    device_name: u(), manufacturer: u(), model: u(), system_type: u(),
    motherboard_manufacturer: u(), motherboard_model: u(), bios_version: u(),
    bios_date: u(), os_edition: u(), os_version: u(), os_build: u(), os_arch: u(),
    install_date: u(), secure_boot: u(), tpm: u(), virtualization: u(),
    uptime_secs: u(), boot_time: u(),
  };
}

export async function getAppInfo(): Promise<AppInfo> {
  if (!isNative) {
    return {
      name: "VYRONIX", brand: "MERQO", version: "0.1.0", build_number: "dev-preview",
      founder: "Shohan Khan", email: "merqoonline@gmail.com",
      product_type: "MERQO Software Product",
      copyright: "Copyright © Shohan Khan. All rights reserved.",
      license: "Proprietary — MERQO. Unauthorized redistribution prohibited.",
      architecture: "x86_64 (Windows 10/11)", tagline: "Know Your Hardware. Trust Your Device.",
    };
  }
  return invoke<AppInfo>("get_app_info");
}

export async function getSystemOverview(): Promise<SystemOverview> {
  if (!isNative) return unavailOverview();
  return invoke<SystemOverview>("get_system_overview");
}

export async function getCpuInfo(): Promise<CpuInfo> {
  if (!isNative) {
    const u = <T,>() => unavailable<T>(SRC);
    return {
      name: u(), manufacturer: u(), architecture: u(), physical_cores: u(),
      logical_processors: u(), base_frequency_mhz: u(), current_frequency_mhz: u(),
      max_frequency_mhz: u(), usage_pct: u(), temperature_c: u(), virtualization: u(),
      l2_cache_kb: u(), l3_cache_kb: u(),
    };
  }
  return invoke<CpuInfo>("get_cpu_info");
}

export async function getGpuInfo(): Promise<GpuInfo[]> {
  if (!isNative) return [];
  return invoke<GpuInfo[]>("get_gpu_info");
}

export async function getMemoryInfo(): Promise<MemoryInfo> {
  if (!isNative) {
    const u = <T,>() => unavailable<T>(SRC);
    return { total_bytes: u(), used_bytes: u(), available_bytes: u(), usage_pct: u(), modules: [] };
  }
  return invoke<MemoryInfo>("get_memory_info");
}

export async function getBatteryInfo(): Promise<BatteryInfo> {
  if (!isNative) {
    const u = <T,>() => unavailable<T>(SRC);
    return {
      present: false, manufacturer: u(), model: u(), chemistry: u(),
      design_capacity_mwh: u(), full_charge_capacity_mwh: u(), remaining_capacity_mwh: u(),
      design_voltage_mv: u(), current_voltage_mv: u(), charge_rate_mw: u(),
      discharge_rate_mw: u(), charging_status: u(), ac_connected: u(), cycle_count: u(),
      health_pct: u(), wear_pct: u(), state_of_charge_pct: u(),
      estimated_runtime_hours: u(), temperature_c: u(),
    };
  }
  return invoke<BatteryInfo>("get_battery_info");
}

export async function getStorageDevices(): Promise<StorageDevice[]> {
  if (!isNative) return [];
  return invoke<StorageDevice[]>("get_storage_devices");
}

export async function getNetworkAdapters(): Promise<NetworkAdapter[]> {
  if (!isNative) return [];
  return invoke<NetworkAdapter[]>("get_network_adapters");
}

export async function getMonitoringSample(): Promise<MonitoringSample | null> {
  if (!isNative) return null;
  return invoke<MonitoringSample>("get_monitoring_sample");
}

export async function runDiagnostics(): Promise<DiagnosticReport> {
  return invoke<DiagnosticReport>("run_diagnostics");
}

export async function runCpuBenchmark(durationSecs: number): Promise<CpuBenchResult> {
  return invoke<CpuBenchResult>("run_cpu_benchmark", { durationSecs });
}

export async function runMemoryBenchmark(sizeMib: number): Promise<MemoryBenchResult> {
  return invoke<MemoryBenchResult>("run_memory_benchmark", { sizeMib });
}

export async function runStorageBenchmark(path: string, sizeMib: number): Promise<StorageBenchResult> {
  return invoke<StorageBenchResult>("run_storage_benchmark", { path, sizeMib });
}

export async function runMemoryTest(sizeMib: number): Promise<MemoryTestResult> {
  return invoke<MemoryTestResult>("run_memory_test", { sizeMib });
}

export async function startStressTest(durationSecs: number, tempLimitC: number): Promise<void> {
  return invoke<void>("start_stress_test", { durationSecs, tempLimitC });
}
export async function stopStressTest(): Promise<void> {
  return invoke<void>("stop_stress_test");
}
export async function pauseStressTest(paused: boolean): Promise<void> {
  return invoke<void>("pause_stress_test", { paused });
}
export async function getStressStatus(): Promise<StressStatus> {
  return invoke<StressStatus>("get_stress_status");
}

export async function getInspectionItems(): Promise<InspectionItem[]> {
  return invoke<InspectionItem[]>("get_inspection_items");
}
export async function setInspectionItem(item: InspectionItem): Promise<void> {
  return invoke<void>("set_inspection_item", { item });
}

export async function getSettings(): Promise<AppSettings> {
  if (!isNative) {
    try {
      const raw = localStorage.getItem("vyronix.settings");
      if (raw) return { ...DEFAULT_SETTINGS, ...JSON.parse(raw) };
    } catch { /* fall through */ }
    return DEFAULT_SETTINGS;
  }
  return invoke<AppSettings>("get_settings");
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  if (!isNative) {
    localStorage.setItem("vyronix.settings", JSON.stringify(settings));
    return;
  }
  return invoke<void>("save_settings", { settings });
}

export interface PingResult {
  host: string;
  transmitted: number;
  received: number;
  loss_pct: number;
  min_ms: number | null;
  avg_ms: number | null;
  max_ms: number | null;
}
export async function pingHost(host: string, count: number): Promise<PingResult> {
  return invoke<PingResult>("ping_host", { host, count });
}
export async function resolveDns(host: string): Promise<string[]> {
  return invoke<string[]>("resolve_dns", { host });
}

export interface WindowsDiagnostics {
  processes: { pid: number; name: string; cpu_pct: number; memory_bytes: number }[];
  services: { name: string; display_name: string; state: string; start_mode: string }[];
  drivers: { name: string; version: string; date: string; status: string; error_code: number }[];
  startup: { name: string; command: string; location: string }[];
  critical_events: { time: string; source: string; event_id: number; message: string }[];
}
export async function getWindowsDiagnostics(): Promise<WindowsDiagnostics> {
  if (!isNative) {
    return { processes: [], services: [], drivers: [], startup: [], critical_events: [] };
  }
  return invoke<WindowsDiagnostics>("get_windows_diagnostics");
}

export type ReportFormat = "pdf" | "html" | "csv" | "json";
export type ReportType = "full" | "hardware" | "battery" | "storage" | "benchmark" | "display" | "inspection" | "summary";

export async function buildReport(reportType: ReportType, includeSensitive: boolean): Promise<unknown> {
  return invoke<unknown>("build_report", { reportType, includeSensitive });
}

export async function saveReportFile(path: string, contents: string, isBase64: boolean, overwrite: boolean): Promise<void> {
  return invoke<void>("save_report_file", { path, contents, isBase64, overwrite });
}

export async function pickSavePath(defaultName: string, extension: string): Promise<string | null> {
  if (!isNative) return null;
  const mod = await import("@tauri-apps/plugin-dialog");
  const result = await mod.save({
    defaultPath: defaultName,
    filters: [{ name: extension.toUpperCase(), extensions: [extension] }],
  });
  return result ?? null;
}

export { NativeUnavailableError };
