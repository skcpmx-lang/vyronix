//! Hardware snapshot types — the typed IPC contract shared with the frontend.
//! Field names match `apps/desktop/src/lib/types.ts` exactly.

use serde::{Deserialize, Serialize};
use vyronix_core::model::{Measurement, Severity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOverview {
    pub device_name: Measurement<String>,
    pub manufacturer: Measurement<String>,
    pub model: Measurement<String>,
    pub system_type: Measurement<String>,
    pub motherboard_manufacturer: Measurement<String>,
    pub motherboard_model: Measurement<String>,
    pub bios_version: Measurement<String>,
    pub bios_date: Measurement<String>,
    pub os_edition: Measurement<String>,
    pub os_version: Measurement<String>,
    pub os_build: Measurement<String>,
    pub os_arch: Measurement<String>,
    pub install_date: Measurement<String>,
    pub secure_boot: Measurement<String>,
    pub tpm: Measurement<String>,
    pub virtualization: Measurement<String>,
    pub uptime_secs: Measurement<u64>,
    pub boot_time: Measurement<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: Measurement<String>,
    pub manufacturer: Measurement<String>,
    pub architecture: Measurement<String>,
    pub physical_cores: Measurement<u32>,
    pub logical_processors: Measurement<u32>,
    pub base_frequency_mhz: Measurement<u64>,
    pub current_frequency_mhz: Measurement<u64>,
    pub max_frequency_mhz: Measurement<u64>,
    pub usage_pct: Measurement<f32>,
    pub temperature_c: Measurement<f32>,
    pub virtualization: Measurement<String>,
    pub l2_cache_kb: Measurement<u32>,
    pub l3_cache_kb: Measurement<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: Measurement<String>,
    pub manufacturer: Measurement<String>,
    pub driver_version: Measurement<String>,
    pub driver_date: Measurement<String>,
    pub dedicated_memory_bytes: Measurement<u64>,
    pub shared_memory_bytes: Measurement<u64>,
    pub current_resolution: Measurement<String>,
    pub refresh_rate_hz: Measurement<u32>,
    pub usage_pct: Measurement<f32>,
    pub temperature_c: Measurement<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryModule {
    pub manufacturer: Measurement<String>,
    pub part_number: Measurement<String>,
    pub capacity_bytes: Measurement<u64>,
    pub speed_mhz: Measurement<u32>,
    pub memory_type: Measurement<String>,
    pub slot: Measurement<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: Measurement<u64>,
    pub used_bytes: Measurement<u64>,
    pub available_bytes: Measurement<u64>,
    pub usage_pct: Measurement<f32>,
    pub modules: Vec<MemoryModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub present: bool,
    pub manufacturer: Measurement<String>,
    pub model: Measurement<String>,
    pub chemistry: Measurement<String>,
    pub design_capacity_mwh: Measurement<u64>,
    pub full_charge_capacity_mwh: Measurement<u64>,
    pub remaining_capacity_mwh: Measurement<u64>,
    pub design_voltage_mv: Measurement<u64>,
    pub current_voltage_mv: Measurement<u64>,
    pub charge_rate_mw: Measurement<u64>,
    pub discharge_rate_mw: Measurement<u64>,
    pub charging_status: Measurement<String>,
    pub ac_connected: Measurement<bool>,
    pub cycle_count: Measurement<u32>,
    pub health_pct: Measurement<f64>,
    pub wear_pct: Measurement<f64>,
    pub state_of_charge_pct: Measurement<f32>,
    pub estimated_runtime_hours: Measurement<f64>,
    pub temperature_c: Measurement<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    pub drive_letter: String,
    pub file_system: Measurement<String>,
    pub capacity_bytes: Measurement<u64>,
    pub free_bytes: Measurement<u64>,
    pub label: Measurement<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub id: String,
    pub model: Measurement<String>,
    pub manufacturer: Measurement<String>,
    pub serial_number: Measurement<String>,
    pub capacity_bytes: Measurement<u64>,
    pub firmware: Measurement<String>,
    pub interface: Measurement<String>,
    pub media_type: Measurement<String>,
    pub health_status: Measurement<String>,
    pub temperature_c: Measurement<f32>,
    pub power_on_hours: Measurement<u64>,
    pub power_cycles: Measurement<u64>,
    pub wear_pct: Measurement<u8>,
    pub available_spare_pct: Measurement<u8>,
    pub percentage_used: Measurement<u8>,
    pub media_errors: Measurement<u64>,
    pub reallocated_sectors: Measurement<u64>,
    pub pending_sectors: Measurement<u64>,
    pub uncorrectable_sectors: Measurement<u64>,
    pub unsafe_shutdowns: Measurement<u64>,
    pub severity: Severity,
    pub smart_notes: Vec<String>,
    pub partitions: Vec<PartitionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapter {
    pub name: Measurement<String>,
    pub adapter_type: Measurement<String>,
    pub mac_address: Measurement<String>,
    pub ip_addresses: Measurement<String>,
    pub link_speed_mbps: Measurement<u64>,
    pub connected: Measurement<bool>,
    pub dns_servers: Measurement<String>,
    pub signal_pct: Measurement<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSample {
    pub timestamp_ms: u64,
    pub cpu_usage_pct: Option<f32>,
    pub memory_used_pct: Option<f32>,
    pub memory_used_bytes: Option<u64>,
    pub cpu_temp_c: Option<f32>,
    pub gpu_usage_pct: Option<f32>,
    pub gpu_temp_c: Option<f32>,
    pub disk_read_bytes_per_s: Option<f64>,
    pub disk_write_bytes_per_s: Option<f64>,
    pub net_rx_bytes_per_s: Option<f64>,
    pub net_tx_bytes_per_s: Option<f64>,
    pub battery_charge_pct: Option<f32>,
    pub battery_rate_mw: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuBenchResult {
    pub single_core_score: f64,
    pub multi_core_score: f64,
    pub integer_ops_per_sec: f64,
    pub float_ops_per_sec: f64,
    pub duration_ms: u64,
    pub threads: u32,
    pub cancelled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBenchResult {
    pub read_bandwidth_mbps: f64,
    pub write_bandwidth_mbps: f64,
    pub copy_bandwidth_mbps: f64,
    pub latency_ns: Option<f64>,
    pub test_size_bytes: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBenchResult {
    pub sequential_read_mbps: f64,
    pub sequential_write_mbps: f64,
    pub random_read_iops: f64,
    pub random_write_iops: f64,
    pub random_read_latency_ms: f64,
    pub random_write_latency_ms: f64,
    pub test_size_bytes: u64,
    pub path: String,
    pub cancelled: bool,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTestResult {
    pub patterns_tested: Vec<String>,
    pub bytes_tested: u64,
    pub errors_found: u64,
    pub first_error_offset: Option<u64>,
    pub duration_ms: u64,
    pub passed: bool,
    pub cancelled: bool,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressStatus {
    pub running: bool,
    pub paused: bool,
    pub elapsed_secs: u64,
    pub target_secs: u64,
    pub cpu_usage_pct: Option<f32>,
    pub cpu_temp_c: Option<f32>,
    pub stopped_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionItem {
    pub id: String,
    pub category: String,
    pub name: String,
    pub status: String,
    pub notes: String,
    pub automated: bool,
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub animations: bool,
    pub notifications: bool,
    pub monitoring_interval_ms: u64,
    pub background_monitoring: bool,
    pub include_sensitive_in_reports: bool,
    pub report_default_format: String,
    pub language: String,
    pub start_minimized: bool,
    pub shortcuts: std::collections::HashMap<String, String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        let mut shortcuts = std::collections::HashMap::new();
        shortcuts.insert("full_scan".into(), "Ctrl+Shift+F".into());
        shortcuts.insert("dashboard".into(), "Ctrl+1".into());
        shortcuts.insert("reports".into(), "Ctrl+R".into());
        shortcuts.insert("settings".into(), "Ctrl+,".into());
        shortcuts.insert("search".into(), "Ctrl+K".into());
        Self {
            theme: "dark".into(),
            animations: true,
            notifications: true,
            monitoring_interval_ms: 1000,
            background_monitoring: false,
            include_sensitive_in_reports: false,
            report_default_format: "pdf".into(),
            language: "en".into(),
            start_minimized: false,
            shortcuts,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub brand: String,
    pub version: String,
    pub build_number: String,
    pub founder: String,
    pub email: String,
    pub product_type: String,
    pub copyright: String,
    pub license: String,
    pub architecture: String,
    pub tagline: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub cpu_pct: f32,
    pub memory_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEntry {
    pub name: String,
    pub display_name: String,
    pub state: String,
    pub start_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverEntry {
    pub name: String,
    pub version: String,
    pub date: String,
    pub status: String,
    pub error_code: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupEntry {
    pub name: String,
    pub command: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEntry {
    pub time: String,
    pub source: String,
    pub event_id: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsDiagnostics {
    pub processes: Vec<ProcessEntry>,
    pub services: Vec<ServiceEntry>,
    pub drivers: Vec<DriverEntry>,
    pub startup: Vec<StartupEntry>,
    pub critical_events: Vec<EventEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResult {
    pub host: String,
    pub transmitted: u32,
    pub received: u32,
    pub loss_pct: f64,
    pub min_ms: Option<f64>,
    pub avg_ms: Option<f64>,
    pub max_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportBundle {
    pub model: vyronix_core::report::ReportModel,
    pub html: String,
    pub csv: String,
    pub json: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticReport {
    pub overall: vyronix_core::scoring::OverallScore,
    pub findings: Vec<vyronix_core::diagnostics::Finding>,
    pub recommendations: Vec<vyronix_core::recommend::Recommendation>,
    pub generated_at_ms: u64,
}

/// Current unix time in milliseconds.
pub fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
