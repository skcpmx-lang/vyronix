//! Shared application state managed by Tauri.

use crate::bench::StressController;
use crate::db::Db;
use crate::hw::monitoring::MonitoringCollector;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub db: Db,
    pub stress: Arc<StressController>,
    pub monitor: Mutex<MonitoringCollector>,
}

impl AppState {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            stress: Arc::new(StressController::new()),
            monitor: Mutex::new(MonitoringCollector::new()),
        }
    }
}
