//! VYRONIX desktop backend entry point. Wires the Tauri builder, plugins,
//! shared state and the full IPC command surface.

mod bench;
mod commands;
mod db;
mod error;
mod hw;
mod logging;
mod model;
mod report;
mod state;

use db::Db;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."));
            let _ = std::fs::create_dir_all(&data_dir);
            logging::init(&data_dir);
            let db = Db::open(&data_dir.join("vyronix.db"))
                .map_err(|e| format!("Failed to open local database: {e}"))?;
            app.manage(AppState::new(db));
            log::info!("VYRONIX backend ready (data dir: {}).", data_dir.display());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_info,
            commands::get_system_overview,
            commands::get_cpu_info,
            commands::get_gpu_info,
            commands::get_memory_info,
            commands::get_battery_info,
            commands::get_storage_devices,
            commands::get_network_adapters,
            commands::get_monitoring_sample,
            commands::run_diagnostics,
            commands::run_cpu_benchmark,
            commands::run_memory_benchmark,
            commands::run_storage_benchmark,
            commands::run_memory_test,
            commands::start_stress_test,
            commands::stop_stress_test,
            commands::pause_stress_test,
            commands::get_stress_status,
            commands::get_inspection_items,
            commands::set_inspection_item,
            commands::get_settings,
            commands::save_settings,
            commands::ping_host,
            commands::resolve_dns,
            commands::get_windows_diagnostics,
            commands::build_report,
            commands::save_report_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running VYRONIX");
}
