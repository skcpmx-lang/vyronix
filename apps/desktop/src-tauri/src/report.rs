//! Report assembly. Builds a core ReportModel from real collected data, runs the
//! transparent diagnostics/scoring/recommendation engine, and serialises the
//! branded bundle (JSON / HTML / CSV). PDF is produced client-side from the model.

use crate::error::AppResult;
use crate::hw;
use crate::model::*;
use vyronix_core::diagnostics::{self, DiagnosticInput};
use vyronix_core::model::{Measurement, Severity, Status};
use vyronix_core::recommend::{self, HardwareFacts};
use vyronix_core::report::{self, DeviceSummary, ReportEntry, ReportModel, ReportSection};
use vyronix_core::scoring::{self, ComponentScore};

fn status_str(s: Status) -> String {
    serde_json::to_value(s).ok().and_then(|v| v.as_str().map(|x| x.to_string())).unwrap_or_else(|| "unavailable".into())
}

fn entry(label: &str, value: String, status: Status, source: &str) -> ReportEntry {
    ReportEntry { label: label.into(), value, status: status_str(status), source: source.into() }
}

fn m_str(label: &str, m: &Measurement<String>) -> ReportEntry {
    entry(label, m.value.clone().unwrap_or_else(|| "—".into()), m.status, &m.source)
}
fn m_num<T: std::fmt::Display>(label: &str, m: &Measurement<T>) -> ReportEntry {
    entry(label, m.value.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "—".into()), m.status, &m.source)
}
fn m_bytes(label: &str, m: &Measurement<u64>) -> ReportEntry {
    entry(label, m.value.map(fmt_bytes).unwrap_or_else(|| "—".into()), m.status, &m.source)
}

fn fmt_bytes(b: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut v = b as f64;
    let mut i = 0;
    while v >= 1024.0 && i < UNITS.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    format!("{:.1} {}", v, UNITS[i])
}

fn system_section(ov: &SystemOverview) -> ReportSection {
    ReportSection {
        title: "System".into(),
        entries: vec![
            m_str("Device name", &ov.device_name), m_str("Manufacturer", &ov.manufacturer),
            m_str("Model", &ov.model), m_str("System type", &ov.system_type),
            m_str("Motherboard", &ov.motherboard_model), m_str("BIOS", &ov.bios_version),
            m_str("OS", &ov.os_edition), m_str("OS version", &ov.os_version),
            m_str("OS build", &ov.os_build), m_str("Architecture", &ov.os_arch),
            m_str("Secure Boot", &ov.secure_boot), m_str("TPM", &ov.tpm),
            m_str("Virtualization", &ov.virtualization), m_num("Uptime (s)", &ov.uptime_secs),
        ],
    }
}

fn cpu_section(c: &CpuInfo) -> ReportSection {
    ReportSection {
        title: "Processor".into(),
        entries: vec![
            m_str("CPU", &c.name), m_str("Manufacturer", &c.manufacturer),
            m_num("Physical cores", &c.physical_cores), m_num("Logical processors", &c.logical_processors),
            m_num("Base clock (MHz)", &c.base_frequency_mhz), m_num("Current clock (MHz)", &c.current_frequency_mhz),
            m_num("Usage (%)", &c.usage_pct), m_num("Temperature (°C)", &c.temperature_c),
            m_str("Virtualization", &c.virtualization),
        ],
    }
}

fn gpu_section(gpus: &[GpuInfo]) -> ReportSection {
    let mut entries = Vec::new();
    for (i, g) in gpus.iter().enumerate() {
        let p = if gpus.len() > 1 { format!("GPU {} — ", i + 1) } else { String::new() };
        entries.push(m_str(&format!("{p}Name"), &g.name));
        entries.push(m_str(&format!("{p}Driver"), &g.driver_version));
        entries.push(m_bytes(&format!("{p}Dedicated memory"), &g.dedicated_memory_bytes));
        entries.push(m_str(&format!("{p}Resolution"), &g.current_resolution));
        entries.push(m_num(&format!("{p}Refresh (Hz)"), &g.refresh_rate_hz));
    }
    if entries.is_empty() {
        entries.push(entry("GPU", "No GPU detected".into(), Status::Unavailable, "WMI"));
    }
    ReportSection { title: "Graphics".into(), entries }
}

fn memory_section(m: &MemoryInfo) -> ReportSection {
    let mut entries = vec![
        m_bytes("Total", &m.total_bytes), m_bytes("Used", &m.used_bytes),
        m_bytes("Available", &m.available_bytes), m_num("Usage (%)", &m.usage_pct),
    ];
    for (i, module) in m.modules.iter().enumerate() {
        let p = format!("DIMM {} — ", i + 1);
        entries.push(m_bytes(&format!("{p}Capacity"), &module.capacity_bytes));
        entries.push(m_num(&format!("{p}Speed (MHz)"), &module.speed_mhz));
        entries.push(m_str(&format!("{p}Type"), &module.memory_type));
        entries.push(m_str(&format!("{p}Slot"), &module.slot));
    }
    ReportSection { title: "Memory".into(), entries }
}

fn battery_section(b: &BatteryInfo) -> ReportSection {
    if !b.present {
        return ReportSection { title: "Battery".into(), entries: vec![entry("Battery", "No battery detected (desktop or not exposed)".into(), Status::Unavailable, "Windows battery interface")] };
    }
    ReportSection {
        title: "Battery".into(),
        entries: vec![
            m_str("Manufacturer", &b.manufacturer), m_str("Model", &b.model), m_str("Chemistry", &b.chemistry),
            m_num("Design capacity (mWh)", &b.design_capacity_mwh), m_num("Full-charge capacity (mWh)", &b.full_charge_capacity_mwh),
            m_num("Health (%)", &b.health_pct), m_num("Wear (%)", &b.wear_pct),
            m_num("Cycle count", &b.cycle_count), m_num("Charge (%)", &b.state_of_charge_pct),
            m_str("Status", &b.charging_status), m_num("Estimated runtime (h)", &b.estimated_runtime_hours),
        ],
    }
}

fn storage_section(devices: &[StorageDevice]) -> ReportSection {
    let mut entries = Vec::new();
    for d in devices {
        let name = d.model.value.clone().unwrap_or_else(|| d.id.clone());
        entries.push(entry(&format!("{name} — capacity"), d.capacity_bytes.value.map(fmt_bytes).unwrap_or_else(|| "—".into()), d.capacity_bytes.status, &d.capacity_bytes.source));
        entries.push(entry(&format!("{name} — media"), d.media_type.value.clone().unwrap_or_else(|| "—".into()), d.media_type.status, &d.media_type.source));
        entries.push(entry(&format!("{name} — health"), d.health_status.value.clone().unwrap_or_else(|| "—".into()), d.health_status.status, &d.health_status.source));
        entries.push(m_num(&format!("{name} — temperature (°C)"), &d.temperature_c));
        entries.push(m_num(&format!("{name} — power-on hours"), &d.power_on_hours));
        entries.push(m_num(&format!("{name} — wear (%)"), &d.wear_pct));
        entries.push(m_num(&format!("{name} — media errors"), &d.media_errors));
    }
    if entries.is_empty() {
        entries.push(entry("Storage", "No storage devices detected".into(), Status::Unavailable, "WMI"));
    }
    ReportSection { title: "Storage".into(), entries }
}

fn network_section(adapters: &[NetworkAdapter]) -> ReportSection {
    let mut entries = Vec::new();
    for a in adapters {
        let name = a.name.value.clone().unwrap_or_else(|| "Adapter".into());
        entries.push(m_str(&format!("{name} — type"), &a.adapter_type));
        entries.push(m_str(&format!("{name} — IP"), &a.ip_addresses));
        entries.push(m_num(&format!("{name} — link (Mbps)"), &a.link_speed_mbps));
        entries.push(m_num(&format!("{name} — connected"), &a.connected));
    }
    if entries.is_empty() {
        entries.push(entry("Network", "No adapters detected".into(), Status::Unavailable, "WMI"));
    }
    ReportSection { title: "Network".into(), entries }
}

fn battery_severity(health: Option<f64>) -> Severity {
    match health {
        Some(h) if h >= 80.0 => Severity::Good,
        Some(h) if h >= 60.0 => Severity::Warning,
        Some(_) => Severity::Critical,
        None => Severity::Unknown,
    }
}
fn thermal_severity(temp: Option<f32>) -> Severity {
    match temp {
        Some(t) if t < 70.0 => Severity::Good,
        Some(t) if t < 85.0 => Severity::Warning,
        Some(_) => Severity::Critical,
        None => Severity::Unknown,
    }
}

/// Build the full report bundle for the requested type.
pub fn build_report(report_type: &str, include_sensitive: bool, inspection: &[InspectionItem], benchmarks: &[String]) -> AppResult<ReportBundle> {
    let ov = hw::system::system_overview();
    let cpu = hw::system::cpu_info();
    let gpus = hw::system::gpu_info();
    let mem = hw::memory_info();
    let bat = hw::battery::battery_info();
    let storage = hw::storage::storage_devices();
    let adapters = hw::network::network_adapters();

    let mut sections: Vec<ReportSection> = Vec::new();
    let title = match report_type {
        "hardware" => "VYRONIX Hardware Report",
        "battery" => "VYRONIX Battery Report",
        "storage" => "VYRONIX Storage Report",
        "benchmark" => "VYRONIX Benchmark Report",
        "inspection" => "VYRONIX Inspection Report",
        "summary" => "VYRONIX Device Summary",
        _ => "VYRONIX Full Diagnostic Report",
    };

    match report_type {
        "battery" => sections.push(battery_section(&bat)),
        "storage" => sections.push(storage_section(&storage)),
        "inspection" => {
            let entries = inspection.iter().map(|i| entry(&format!("{} — {}", i.category, i.name), format!("{}{}", i.status, if i.notes.is_empty() { String::new() } else { format!(" ({})", i.notes) }), Status::NotTested, "Guided inspection")).collect();
            sections.push(ReportSection { title: "Inspection checklist".into(), entries });
        }
        "benchmark" => {
            let entries = benchmarks.iter().enumerate().map(|(i, b)| entry(&format!("Run {}", i + 1), b.clone(), Status::Available, "Measured benchmark")).collect();
            sections.push(ReportSection { title: "Benchmark results (most recent first)".into(), entries });
        }
        "summary" => {
            sections.push(system_section(&ov));
            sections.push(cpu_section(&cpu));
            sections.push(memory_section(&mem));
            sections.push(battery_section(&bat));
            sections.push(storage_section(&storage));
        }
        "hardware" => {
            sections.push(system_section(&ov));
            sections.push(cpu_section(&cpu));
            sections.push(gpu_section(&gpus));
            sections.push(memory_section(&mem));
            sections.push(battery_section(&bat));
            sections.push(storage_section(&storage));
            sections.push(network_section(&adapters));
        }
        _ => {
            sections.push(system_section(&ov));
            sections.push(cpu_section(&cpu));
            sections.push(gpu_section(&gpus));
            sections.push(memory_section(&mem));
            sections.push(battery_section(&bat));
            sections.push(storage_section(&storage));
            sections.push(network_section(&adapters));
        }
    }

    // Diagnostics + scoring + recommendations (shared with run_diagnostics).
    let assessment = assess(&cpu, &gpus, &mem, &bat, &storage, &adapters, inspection);
    let findings = assessment.findings;
    let overall = assessment.overall;
    let recommendations = assessment.recommendations;

    let device = DeviceSummary {
        device_name: ov.device_name.value.clone().unwrap_or_else(|| "Unknown".into()),
        manufacturer: ov.manufacturer.value.clone().unwrap_or_else(|| "Unknown".into()),
        model: ov.model.value.clone().unwrap_or_else(|| "Unknown".into()),
        os: ov.os_edition.value.clone().unwrap_or_else(|| "Unknown".into()),
    };

    let model = ReportModel {
        title: title.into(),
        report_type: report_type.into(),
        generated_at_ms: now_ms(),
        app_version: env!("CARGO_PKG_VERSION").into(),
        device,
        sections,
        findings,
        overall: Some(overall),
        recommendations,
        include_sensitive,
    };

    let json = model.to_json()?;
    let html = report::to_html(&model);
    let csv = report::to_csv(&model);
    Ok(ReportBundle { model, html, csv, json })
}

/// The transparent diagnostic assessment: findings + weighted score + recommendations.
pub struct Assessment {
    pub findings: Vec<vyronix_core::diagnostics::Finding>,
    pub overall: vyronix_core::scoring::OverallScore,
    pub recommendations: Vec<vyronix_core::recommend::Recommendation>,
}

pub fn assess(
    cpu: &CpuInfo,
    gpus: &[GpuInfo],
    mem: &MemoryInfo,
    bat: &BatteryInfo,
    storage: &[StorageDevice],
    adapters: &[NetworkAdapter],
    inspection: &[InspectionItem],
) -> Assessment {
    let storage_severity = storage.iter().map(|d| d.severity).filter(|s| *s != Severity::Unknown).max_by_key(|s| sev_rank(*s));
    let storage_temp = storage.iter().filter_map(|d| d.temperature_c.value).fold(None, |acc: Option<f32>, t| Some(acc.map_or(t, |m| m.max(t))));
    let input = DiagnosticInput {
        battery_health_pct: bat.health_pct.value,
        battery_present: bat.present,
        storage_severity,
        storage_label: storage.first().and_then(|d| d.model.value.clone()).unwrap_or_else(|| "Storage".into()),
        memory_used_pct: mem.usage_pct.value.map(|v| v as f64),
        cpu_temp_c: cpu.temperature_c.value.map(|v| v as f64),
        gpu_temp_c: gpus.iter().filter_map(|g| g.temperature_c.value).next().map(|v| v as f64),
        storage_temp_c: storage_temp.map(|v| v as f64),
        driver_problem_count: None,
        critical_event_count: None,
        network_connected: adapters.iter().filter_map(|a| a.connected.value).next(),
        failed_test_count: inspection.iter().filter(|i| i.status == "failed").count() as u32,
    };
    let findings = diagnostics::evaluate(&input);

    let components: Vec<ComponentScore> = vec![
        scoring::component("battery", "Battery health", 2.0, battery_severity(bat.health_pct.value), &format!("Health {}", bat.health_pct.value.map(|h| format!("{h:.0}%")).unwrap_or_else(|| "not measurable".into()))),
        scoring::component("storage", "Storage health", 3.0, storage_severity.unwrap_or(Severity::Unknown), "SMART-derived health from the Windows storage interface"),
        scoring::component("cpu_thermal", "CPU thermals", 1.5, thermal_severity(cpu.temperature_c.value), "CPU temperature sensor (if exposed)"),
        scoring::component("gpu_thermal", "GPU thermals", 1.0, thermal_severity(gpus.iter().filter_map(|g| g.temperature_c.value).next()), "GPU temperature sensor (if exposed)"),
        scoring::component("storage_thermal", "Storage thermals", 1.0, thermal_severity(storage_temp), "Storage temperature (if exposed)"),
    ];
    let overall = scoring::compute_overall(components);

    let facts = HardwareFacts {
        logical_processors: cpu.logical_processors.value,
        physical_cores: cpu.physical_cores.value,
        total_ram_bytes: mem.total_bytes.value,
        storage_is_ssd: storage.iter().find_map(|d| d.media_type.value.as_ref().map(|m| m.contains("SSD"))),
        gpu_dedicated: gpus.first().and_then(|g| g.dedicated_memory_bytes.value.map(|v| v > 0)),
        gpu_name: gpus.first().and_then(|g| g.name.value.clone()),
        battery_health_pct: bat.health_pct.value,
    };
    let recommendations = recommend::recommend(&facts);

    Assessment { findings, overall, recommendations }
}

fn sev_rank(s: Severity) -> u8 {
    match s {
        Severity::Critical => 4,
        Severity::Warning => 3,
        Severity::Information => 2,
        Severity::Good => 1,
        Severity::Unknown => 0,
    }
}
