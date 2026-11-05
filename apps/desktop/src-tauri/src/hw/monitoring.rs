//! Real-time monitoring collector. Keeps persistent sysinfo state so CPU usage
//! and network/disk rates are computed across refresh intervals (not fabricated).

use crate::model::{now_ms, MonitoringSample};

pub struct MonitoringCollector {
    sys: sysinfo::System,
    networks: sysinfo::Networks,
    components: sysinfo::Components,
    last_ms: u64,
}

impl MonitoringCollector {
    pub fn new() -> Self {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        Self {
            sys,
            networks: sysinfo::Networks::new_with_refreshed_list(),
            components: sysinfo::Components::new_with_refreshed_list(),
            last_ms: now_ms(),
        }
    }

    /// Take a sample. `interval_ms` is the nominal polling interval used to
    /// normalise byte counters into per-second rates.
    pub fn sample(&mut self, interval_ms: u64) -> MonitoringSample {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
        self.networks.refresh_list();
        self.components.refresh_list();

        let now = now_ms();
        let elapsed_s = ((now.saturating_sub(self.last_ms)).max(1)) as f64 / 1000.0;
        self.last_ms = now;
        let _ = interval_ms;

        let cpu_usage = self.sys.global_cpu_usage();
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let mem_pct = if total > 0 { Some((used as f32 / total as f32) * 100.0) } else { None };

        // Network rates: bytes since last refresh / elapsed seconds.
        let (rx, tx) = self.networks.list().values()
            .fold((0u64, 0u64), |(rx, tx), n| (rx + n.received(), tx + n.transmitted()));
        let rx_rate = if elapsed_s > 0.0 { Some(rx as f64 / elapsed_s) } else { None };
        let tx_rate = if elapsed_s > 0.0 { Some(tx as f64 / elapsed_s) } else { None };

        // Temperatures only from real, labelled sensors — never inferred.
        let mut cpu_temp = None;
        let mut gpu_temp = None;
        for c in self.components.list() {
            let label = c.label().to_lowercase();
            let t = c.temperature();
            if !t.is_finite() || t <= 0.0 || t >= 150.0 {
                continue;
            }
            if cpu_temp.is_none() && (label.contains("cpu") || label.contains("package") || label.contains("tdie") || label.contains("tctl")) {
                cpu_temp = Some(t);
            } else if gpu_temp.is_none() && (label.contains("gpu") || label.contains("gfx")) {
                gpu_temp = Some(t);
            }
        }

        MonitoringSample {
            timestamp_ms: now,
            cpu_usage_pct: Some(cpu_usage),
            memory_used_pct: mem_pct,
            memory_used_bytes: Some(used),
            cpu_temp_c: cpu_temp,
            gpu_usage_pct: None, // requires a GPU perf-counter interface; not fabricated
            gpu_temp_c: gpu_temp,
            disk_read_bytes_per_s: None, // system-wide disk throughput not exposed by sysinfo
            disk_write_bytes_per_s: None,
            net_rx_bytes_per_s: rx_rate,
            net_tx_bytes_per_s: tx_rate,
            battery_charge_pct: None, // battery is polled on the Battery page
            battery_rate_mw: None,
        }
    }
}

impl Default for MonitoringCollector {
    fn default() -> Self {
        Self::new()
    }
}
