// TypeScript mirrors of the Rust vyronix-core data model.
// These are the typed IPC contract between the frontend and native backend.

export type Status =
  | "available"
  | "verified"
  | "estimated"
  | "unsupported"
  | "permission_required"
  | "unavailable"
  | "failed"
  | "not_tested"
  | "manual_verification_required";

export type Confidence = "high" | "medium" | "low" | "none";

export type Severity = "good" | "information" | "warning" | "critical" | "unknown";

export interface Measurement<T> {
  value: T | null;
  status: Status;
  confidence: Confidence;
  source: string;
  note?: string | null;
  timestamp_ms: number;
}

export const STATUS_LABELS: Record<Status, string> = {
  available: "Available",
  verified: "Verified",
  estimated: "Estimated",
  unsupported: "Unsupported",
  permission_required: "Permission Required",
  unavailable: "Unavailable",
  failed: "Failed",
  not_tested: "Not Tested",
  manual_verification_required: "Manual Verification Required",
};

export const SEVERITY_LABELS: Record<Severity, string> = {
  good: "Good",
  information: "Information",
  warning: "Warning",
  critical: "Critical",
  unknown: "Unknown",
};

// ---- Hardware snapshots ----

export interface SystemOverview {
  device_name: Measurement<string>;
  manufacturer: Measurement<string>;
  model: Measurement<string>;
  system_type: Measurement<string>;
  motherboard_manufacturer: Measurement<string>;
  motherboard_model: Measurement<string>;
  bios_version: Measurement<string>;
  bios_date: Measurement<string>;
  os_edition: Measurement<string>;
  os_version: Measurement<string>;
  os_build: Measurement<string>;
  os_arch: Measurement<string>;
  install_date: Measurement<string>;
  secure_boot: Measurement<string>;
  tpm: Measurement<string>;
  virtualization: Measurement<string>;
  uptime_secs: Measurement<number>;
  boot_time: Measurement<string>;
}

export interface CpuInfo {
  name: Measurement<string>;
  manufacturer: Measurement<string>;
  architecture: Measurement<string>;
  physical_cores: Measurement<number>;
  logical_processors: Measurement<number>;
  base_frequency_mhz: Measurement<number>;
  current_frequency_mhz: Measurement<number>;
  max_frequency_mhz: Measurement<number>;
  usage_pct: Measurement<number>;
  temperature_c: Measurement<number>;
  virtualization: Measurement<string>;
  l2_cache_kb: Measurement<number>;
  l3_cache_kb: Measurement<number>;
}

export interface GpuInfo {
  name: Measurement<string>;
  manufacturer: Measurement<string>;
  driver_version: Measurement<string>;
  driver_date: Measurement<string>;
  dedicated_memory_bytes: Measurement<number>;
  shared_memory_bytes: Measurement<number>;
  current_resolution: Measurement<string>;
  refresh_rate_hz: Measurement<number>;
  usage_pct: Measurement<number>;
  temperature_c: Measurement<number>;
}

export interface MemoryModule {
  manufacturer: Measurement<string>;
  part_number: Measurement<string>;
  capacity_bytes: Measurement<number>;
  speed_mhz: Measurement<number>;
  memory_type: Measurement<string>;
  slot: Measurement<string>;
}

export interface MemoryInfo {
  total_bytes: Measurement<number>;
  used_bytes: Measurement<number>;
  available_bytes: Measurement<number>;
  usage_pct: Measurement<number>;
  modules: MemoryModule[];
}

export interface BatteryInfo {
  present: boolean;
  manufacturer: Measurement<string>;
  model: Measurement<string>;
  chemistry: Measurement<string>;
  design_capacity_mwh: Measurement<number>;
  full_charge_capacity_mwh: Measurement<number>;
  remaining_capacity_mwh: Measurement<number>;
  design_voltage_mv: Measurement<number>;
  current_voltage_mv: Measurement<number>;
  charge_rate_mw: Measurement<number>;
  discharge_rate_mw: Measurement<number>;
  charging_status: Measurement<string>;
  ac_connected: Measurement<boolean>;
  cycle_count: Measurement<number>;
  health_pct: Measurement<number>;
  wear_pct: Measurement<number>;
  state_of_charge_pct: Measurement<number>;
  estimated_runtime_hours: Measurement<number>;
  temperature_c: Measurement<number>;
}

export interface StorageDevice {
  id: string;
  model: Measurement<string>;
  manufacturer: Measurement<string>;
  serial_number: Measurement<string>;
  capacity_bytes: Measurement<number>;
  firmware: Measurement<string>;
  interface: Measurement<string>;
  media_type: Measurement<string>;
  health_status: Measurement<string>;
  temperature_c: Measurement<number>;
  power_on_hours: Measurement<number>;
  power_cycles: Measurement<number>;
  wear_pct: Measurement<number>;
  available_spare_pct: Measurement<number>;
  percentage_used: Measurement<number>;
  media_errors: Measurement<number>;
  reallocated_sectors: Measurement<number>;
  pending_sectors: Measurement<number>;
  uncorrectable_sectors: Measurement<number>;
  unsafe_shutdowns: Measurement<number>;
  severity: Severity;
  smart_notes: string[];
  partitions: PartitionInfo[];
}

export interface PartitionInfo {
  drive_letter: string;
  file_system: Measurement<string>;
  capacity_bytes: Measurement<number>;
  free_bytes: Measurement<number>;
  label: Measurement<string>;
}

export interface NetworkAdapter {
  name: Measurement<string>;
  adapter_type: Measurement<string>;
  mac_address: Measurement<string>;
  ip_addresses: Measurement<string>;
  link_speed_mbps: Measurement<number>;
  connected: Measurement<boolean>;
  dns_servers: Measurement<string>;
  signal_pct: Measurement<number>;
}

export interface MonitoringSample {
  timestamp_ms: number;
  cpu_usage_pct: number | null;
  memory_used_pct: number | null;
  memory_used_bytes: number | null;
  cpu_temp_c: number | null;
  gpu_usage_pct: number | null;
  gpu_temp_c: number | null;
  disk_read_bytes_per_s: number | null;
  disk_write_bytes_per_s: number | null;
  net_rx_bytes_per_s: number | null;
  net_tx_bytes_per_s: number | null;
  battery_charge_pct: number | null;
  battery_rate_mw: number | null;
}

export interface Finding {
  id: string;
  category: string;
  finding: string;
  severity: Severity;
  evidence: string;
  data_source: string;
  explanation: string;
  recommendation: string;
  confidence: Confidence;
  timestamp_ms: number;
}

export interface ComponentScore {
  id: string;
  label: string;
  weight: number;
  score: number | null;
  severity: Severity;
  included: boolean;
  rationale: string;
}

export interface OverallScore {
  score: number | null;
  coverage: number;
  included_count: number;
  total_count: number;
  confidence: "high" | "medium" | "low" | "insufficient";
  components: ComponentScore[];
}

export interface Recommendation {
  use_case: string;
  suitability: "well_suited" | "adequate" | "limited" | "not_recommended" | "unknown";
  rationale: string;
}

export interface DiagnosticReport {
  overall: OverallScore;
  findings: Finding[];
  recommendations: Recommendation[];
  generated_at_ms: number;
}

// ---- Benchmarks ----

export interface CpuBenchResult {
  single_core_score: number;
  multi_core_score: number;
  integer_ops_per_sec: number;
  float_ops_per_sec: number;
  duration_ms: number;
  threads: number;
  cancelled: boolean;
}

export interface MemoryBenchResult {
  read_bandwidth_mbps: number;
  write_bandwidth_mbps: number;
  copy_bandwidth_mbps: number;
  latency_ns: number | null;
  test_size_bytes: number;
  duration_ms: number;
}

export interface StorageBenchResult {
  sequential_read_mbps: number;
  sequential_write_mbps: number;
  random_read_iops: number;
  random_write_iops: number;
  random_read_latency_ms: number;
  random_write_latency_ms: number;
  test_size_bytes: number;
  path: string;
  cancelled: boolean;
  duration_ms: number;
}

export interface MemoryTestResult {
  patterns_tested: string[];
  bytes_tested: number;
  errors_found: number;
  first_error_offset: number | null;
  duration_ms: number;
  passed: boolean;
  cancelled: boolean;
  note: string;
}

export interface StressStatus {
  running: boolean;
  paused: boolean;
  elapsed_secs: number;
  target_secs: number;
  cpu_usage_pct: number | null;
  cpu_temp_c: number | null;
  stopped_reason: string | null;
}

// ---- Inspection ----

export type InspectionStatus =
  | "not_tested"
  | "passed"
  | "warning"
  | "failed"
  | "unsupported"
  | "manual_verification_required";

export interface InspectionItem {
  id: string;
  category: string;
  name: string;
  status: InspectionStatus;
  notes: string;
  automated: boolean;
  updated_at_ms: number;
}

// ---- Settings ----

export interface AppSettings {
  theme: "dark" | "light" | "system";
  animations: boolean;
  notifications: boolean;
  monitoring_interval_ms: number;
  background_monitoring: boolean;
  include_sensitive_in_reports: boolean;
  report_default_format: "pdf" | "html" | "csv" | "json";
  language: string;
  start_minimized: boolean;
  shortcuts: Record<string, string>;
}

export const DEFAULT_SETTINGS: AppSettings = {
  theme: "dark",
  animations: true,
  notifications: true,
  monitoring_interval_ms: 1000,
  background_monitoring: false,
  include_sensitive_in_reports: false,
  report_default_format: "pdf",
  language: "en",
  start_minimized: false,
  shortcuts: {
    full_scan: "Ctrl+Shift+F",
    dashboard: "Ctrl+1",
    reports: "Ctrl+R",
    settings: "Ctrl+,",
    search: "Ctrl+K",
  },
};

// ---- Report model (mirrors Rust vyronix-core::report) ----

export interface ReportEntry {
  label: string;
  value: string;
  status: string;
  source: string;
}

export interface ReportSection {
  title: string;
  entries: ReportEntry[];
}

export interface DeviceSummary {
  device_name: string;
  manufacturer: string;
  model: string;
  os: string;
}

export interface ReportModel {
  title: string;
  report_type: string;
  generated_at_ms: number;
  app_version: string;
  device: DeviceSummary;
  sections: ReportSection[];
  findings: Finding[];
  overall: OverallScore | null;
  recommendations: Recommendation[];
  include_sensitive: boolean;
}

export interface ReportBundle {
  model: ReportModel;
  html: string;
  csv: string;
  json: string;
}

export interface AppInfo {
  name: string;
  brand: string;
  version: string;
  build_number: string;
  founder: string;
  email: string;
  product_type: string;
  copyright: string;
  license: string;
  architecture: string;
  tagline: string;
}
