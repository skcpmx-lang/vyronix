//! Tauri IPC command surface. Argument names are snake_case; Tauri maps the
//! frontend's camelCase automatically. Every command returns real data or an
//! honest error — never fabricated values.

use crate::error::{AppError, AppResult};
use crate::model::*;
use crate::state::AppState;
use crate::{bench, hw, report};
use base64::Engine;
use std::net::ToSocketAddrs;
use tauri::State;

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: "VYRONIX".into(),
        brand: "MERQO".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        build_number: option_env!("VYRONIX_BUILD_NUMBER").unwrap_or("dev").into(),
        founder: "Shohan Khan".into(),
        email: "merqoonline@gmail.com".into(),
        product_type: "Windows PC & Laptop Diagnostic Ecosystem".into(),
        copyright: "Copyright © Shohan Khan. All rights reserved.".into(),
        license: "Proprietary — MERQO".into(),
        architecture: "x86_64 (Windows 10/11)".into(),
        tagline: "Know Your Hardware. Trust Your Device.".into(),
    }
}

#[tauri::command]
pub fn get_system_overview() -> SystemOverview {
    hw::system::system_overview()
}

#[tauri::command]
pub fn get_cpu_info() -> CpuInfo {
    hw::system::cpu_info()
}

#[tauri::command]
pub fn get_gpu_info() -> Vec<GpuInfo> {
    hw::system::gpu_info()
}

#[tauri::command]
pub fn get_memory_info() -> MemoryInfo {
    hw::system::memory_info()
}

#[tauri::command]
pub fn get_battery_info() -> BatteryInfo {
    hw::battery::battery_info()
}

#[tauri::command]
pub fn get_storage_devices() -> Vec<StorageDevice> {
    hw::storage::storage_devices()
}

#[tauri::command]
pub fn get_network_adapters() -> Vec<NetworkAdapter> {
    hw::network::network_adapters()
}

#[tauri::command]
pub fn get_monitoring_sample(state: State<'_, AppState>) -> MonitoringSample {
    let interval = state.db.load_settings().map(|s| s.monitoring_interval_ms).unwrap_or(1000);
    let mut monitor = state.monitor.lock().unwrap();
    monitor.sample(interval)
}

#[tauri::command]
pub fn run_diagnostics(state: State<'_, AppState>) -> AppResult<DiagnosticReport> {
    let cpu = hw::system::cpu_info();
    let gpus = hw::system::gpu_info();
    let mem = hw::system::memory_info();
    let bat = hw::battery::battery_info();
    let storage = hw::storage::storage_devices();
    let adapters = hw::network::network_adapters();
    let inspection = state.db.load_inspection().unwrap_or_default();
    let assessment = report::assess(&cpu, &gpus, &mem, &bat, &storage, &adapters, &inspection);
    let report = DiagnosticReport {
        overall: assessment.overall,
        findings: assessment.findings,
        recommendations: assessment.recommendations,
        generated_at_ms: now_ms(),
    };
    if let Ok(json) = serde_json::to_string(&report) {
        let _ = state.db.save_diagnostic(&json);
    }
    Ok(report)
}

#[tauri::command]
pub async fn run_cpu_benchmark(state: State<'_, AppState>, duration_secs: u64) -> AppResult<CpuBenchResult> {
    let result = tauri::async_runtime::spawn_blocking(move || bench::cpu_benchmark(duration_secs))
        .await
        .map_err(|e| AppError::msg(format!("Benchmark task failed: {e}")))?;
    if let Ok(json) = serde_json::to_string(&result) {
        let _ = state.db.save_benchmark("cpu", &json);
    }
    Ok(result)
}

#[tauri::command]
pub async fn run_memory_benchmark(state: State<'_, AppState>, size_mib: u64) -> AppResult<MemoryBenchResult> {
    let result = tauri::async_runtime::spawn_blocking(move || bench::memory_benchmark(size_mib))
        .await
        .map_err(|e| AppError::msg(format!("Benchmark task failed: {e}")))?;
    if let Ok(json) = serde_json::to_string(&result) {
        let _ = state.db.save_benchmark("memory", &json);
    }
    Ok(result)
}

#[tauri::command]
pub async fn run_storage_benchmark(state: State<'_, AppState>, path: String, size_mib: u64) -> AppResult<StorageBenchResult> {
    let result = tauri::async_runtime::spawn_blocking(move || bench::storage_benchmark(&path, size_mib))
        .await
        .map_err(|e| AppError::msg(format!("Benchmark task failed: {e}")))?
        .map_err(AppError::msg)?;
    if let Ok(json) = serde_json::to_string(&result) {
        let _ = state.db.save_benchmark("storage", &json);
    }
    Ok(result)
}

#[tauri::command]
pub async fn run_memory_test(size_mib: u64) -> AppResult<MemoryTestResult> {
    let result = tauri::async_runtime::spawn_blocking(move || bench::memory_test(size_mib))
        .await
        .map_err(|e| AppError::msg(format!("Memory test task failed: {e}")))?;
    Ok(result)
}

#[tauri::command]
pub fn start_stress_test(state: State<'_, AppState>, duration_secs: u64, temp_limit_c: f32) -> AppResult<()> {
    bench::start_stress(&state.stress, duration_secs, temp_limit_c).map_err(AppError::msg)
}

#[tauri::command]
pub fn stop_stress_test(state: State<'_, AppState>) {
    state.stress.stop.store(true, std::sync::atomic::Ordering::SeqCst);
}

#[tauri::command]
pub fn pause_stress_test(state: State<'_, AppState>, paused: bool) {
    state.stress.paused.store(paused, std::sync::atomic::Ordering::SeqCst);
}

#[tauri::command]
pub fn get_stress_status(state: State<'_, AppState>) -> StressStatus {
    bench::stress_status(&state.stress)
}

#[tauri::command]
pub fn get_inspection_items(state: State<'_, AppState>) -> AppResult<Vec<InspectionItem>> {
    state.db.load_inspection()
}

#[tauri::command]
pub fn set_inspection_item(state: State<'_, AppState>, item: InspectionItem) -> AppResult<()> {
    state.db.upsert_inspection(&item)
}

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> AppResult<AppSettings> {
    state.db.load_settings()
}

#[tauri::command]
pub fn save_settings(state: State<'_, AppState>, settings: AppSettings) -> AppResult<()> {
    state.db.save_settings(&settings)
}

#[tauri::command]
pub fn ping_host(host: String, count: u32) -> AppResult<PingResult> {
    hw::network::ping(&host, count).map_err(AppError::msg)
}

#[tauri::command]
pub fn resolve_dns(host: String) -> AppResult<Vec<String>> {
    let addrs = (host.as_str(), 0)
        .to_socket_addrs()
        .map_err(|e| AppError::msg(format!("DNS resolution failed for '{host}': {e}")))?;
    let mut ips: Vec<String> = addrs.map(|a| a.ip().to_string()).collect();
    ips.sort();
    ips.dedup();
    if ips.is_empty() {
        return Err(AppError::msg(format!("No addresses found for '{host}'.")));
    }
    Ok(ips)
}

#[tauri::command]
pub fn get_windows_diagnostics() -> WindowsDiagnostics {
    hw::windows::windows_diagnostics()
}

#[tauri::command]
pub async fn build_report(state: State<'_, AppState>, report_type: String, include_sensitive: bool) -> AppResult<ReportBundle> {
    let inspection = state.db.load_inspection().unwrap_or_default();
    let benchmarks = load_recent_benchmarks(&state);
    let bundle = tauri::async_runtime::spawn_blocking(move || {
        report::build_report(&report_type, include_sensitive, &inspection, &benchmarks)
    })
    .await
    .map_err(|e| AppError::msg(format!("Report task failed: {e}")))??;
    Ok(bundle)
}

fn load_recent_benchmarks(state: &State<'_, AppState>) -> Vec<String> {
    state.db.recent_benchmarks(20).unwrap_or_default()
}

#[tauri::command]
pub fn save_report_file(path: String, contents: String, is_base64: bool, overwrite: bool) -> AppResult<()> {
    let target = std::path::Path::new(&path);
    // Safety: refuse to overwrite an existing file unless explicitly allowed.
    if target.exists() && !overwrite {
        return Err(AppError::Validation(format!("'{}' already exists. Choose a different name or enable overwrite.", path)));
    }
    if let Some(parent) = target.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let bytes = if is_base64 {
        base64::engine::general_purpose::STANDARD
            .decode(contents.as_bytes())
            .map_err(|e| AppError::Validation(format!("Invalid base64 report payload: {e}")))?
    } else {
        contents.into_bytes()
    };
    std::fs::write(target, bytes)?;
    Ok(())
}
