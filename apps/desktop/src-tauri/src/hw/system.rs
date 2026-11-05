//! System overview, CPU, GPU and memory collection.
//!
//! Sources: sysinfo (cross-platform) + WMI/CIM (Windows). Every field is a
//! `Measurement` with provenance; unavailable data is never fabricated.

use crate::model::*;
use vyronix_core::model::{Confidence, Measurement, Status};

const SYSINFO: &str = "sysinfo";

fn unavail_str(note: &str) -> Measurement<String> {
    Measurement::unavailable("n/a", note)
}

// ============================ Windows implementation ============================

#[cfg(windows)]
pub fn system_overview() -> SystemOverview {
    use super::wmi;
    let cs = wmi::cimv2("SELECT Manufacturer, Model, SystemType FROM Win32_ComputerSystem")
        .ok()
        .and_then(|r| r.into_iter().next());
    let bb = wmi::cimv2("SELECT Manufacturer, Product FROM Win32_BaseBoard")
        .ok()
        .and_then(|r| r.into_iter().next());
    let bios = wmi::cimv2("SELECT SMBIOSBIOSVersion, ReleaseDate FROM Win32_BIOS")
        .ok()
        .and_then(|r| r.into_iter().next());
    let os = wmi::cimv2("SELECT Caption, Version, BuildNumber, OSArchitecture, InstallDate FROM Win32_OperatingSystem").ok().and_then(|r| r.into_iter().next());
    let cpu = wmi::cimv2("SELECT VirtualizationFirmwareEnabled FROM Win32_Processor")
        .ok()
        .and_then(|r| r.into_iter().next());

    let m_str = |row: &Option<wmi::WmiRow>, key: &str, src: &str| -> Measurement<String> {
        match row.as_ref().and_then(|r| wmi::get_str(r, key)) {
            Some(v) => Measurement::available(v, src),
            None => Measurement::unavailable(src, "Not exposed by the hardware interface."),
        }
    };

    let device_name = sysinfo::System::host_name()
        .map(|v| Measurement::available(v, SYSINFO))
        .unwrap_or_else(|| unavail_str("Host name unavailable."));
    let uptime = Measurement::available(sysinfo::System::uptime(), SYSINFO);
    let boot = Measurement::available(format_epoch(sysinfo::System::boot_time()), SYSINFO);
    let arch = sysinfo::System::cpu_arch()
        .map(|v| Measurement::available(v, SYSINFO))
        .unwrap_or_else(|| unavail_str("Architecture unavailable."));

    let virtualization = match cpu
        .as_ref()
        .and_then(|r| wmi::get_bool(r, "VirtualizationFirmwareEnabled"))
    {
        Some(true) => {
            Measurement::available("Enabled (firmware)".to_string(), "WMI Win32_Processor")
        }
        Some(false) => {
            Measurement::available("Disabled (firmware)".to_string(), "WMI Win32_Processor")
        }
        None => {
            Measurement::unavailable("WMI Win32_Processor", "Virtualization status not exposed.")
        }
    };

    SystemOverview {
        device_name,
        manufacturer: m_str(&cs, "Manufacturer", "WMI Win32_ComputerSystem"),
        model: m_str(&cs, "Model", "WMI Win32_ComputerSystem"),
        system_type: m_str(&cs, "SystemType", "WMI Win32_ComputerSystem"),
        motherboard_manufacturer: m_str(&bb, "Manufacturer", "WMI Win32_BaseBoard"),
        motherboard_model: m_str(&bb, "Product", "WMI Win32_BaseBoard"),
        bios_version: m_str(&bios, "SMBIOSBIOSVersion", "WMI Win32_BIOS"),
        bios_date: m_str(&bios, "ReleaseDate", "WMI Win32_BIOS"),
        os_edition: m_str(&os, "Caption", "WMI Win32_OperatingSystem"),
        os_version: m_str(&os, "Version", "WMI Win32_OperatingSystem"),
        os_build: m_str(&os, "BuildNumber", "WMI Win32_OperatingSystem"),
        os_arch: arch,
        install_date: m_str(&os, "InstallDate", "WMI Win32_OperatingSystem"),
        secure_boot: read_secure_boot(),
        tpm: read_tpm(),
        virtualization,
        uptime_secs: uptime,
        boot_time: boot,
    }
}

#[cfg(windows)]
fn read_secure_boot() -> Measurement<String> {
    // Read-only registry query via reg.exe (fixed arguments, no user input).
    let out = std::process::Command::new("reg")
        .args([
            "query",
            "HKLM\\SYSTEM\\CurrentControlSet\\Control\\SecureBoot\\State",
            "/v",
            "UEFISecureBootEnabled",
        ])
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            if let Some(line) = text.lines().find(|l| l.contains("UEFISecureBootEnabled")) {
                let enabled = line.trim().ends_with("0x1");
                Measurement::available(
                    if enabled {
                        "Enabled".into()
                    } else {
                        "Disabled".into()
                    },
                    "Windows registry (SecureBoot\\State)",
                )
            } else {
                Measurement::unavailable("Windows registry", "Secure Boot value not found.")
            }
        }
        _ => Measurement::unsupported(
            "Windows registry",
            "Secure Boot state not present (likely legacy BIOS).",
        ),
    }
}

#[cfg(windows)]
fn read_tpm() -> Measurement<String> {
    use super::wmi;
    match wmi::query_json(
        Some("root\\cimv2\\Security\\MicrosoftTpm"),
        "SELECT IsEnabled_InitialValue, SpecVersion FROM Win32_Tpm",
    ) {
        Ok(rows) => match rows.into_iter().next() {
            Some(row) => {
                let enabled = wmi::get_bool(&row, "IsEnabled_InitialValue").unwrap_or(false);
                let spec =
                    wmi::get_str(&row, "SpecVersion").unwrap_or_else(|| "unknown spec".into());
                Measurement::available(
                    format!(
                        "{} (spec {})",
                        if enabled {
                            "Present & enabled"
                        } else {
                            "Present, not enabled"
                        },
                        spec
                    ),
                    "WMI Win32_Tpm",
                )
            }
            None => Measurement::unsupported("WMI Win32_Tpm", "No TPM instance reported."),
        },
        Err(_) => Measurement::permission_required(
            "WMI Win32_Tpm",
            "TPM namespace requires administrator access.",
        ),
    }
}

#[cfg(windows)]
pub fn cpu_info() -> CpuInfo {
    use super::wmi;
    let mut sys = sysinfo::System::new_all();
    sys.refresh_cpu_usage();
    sys.refresh_cpu_frequency();

    let wmi_cpu = wmi::cimv2("SELECT MaxClockSpeed, L2CacheSize, L3CacheSize, VirtualizationFirmwareEnabled FROM Win32_Processor").ok().and_then(|r| r.into_iter().next());
    let first = sys.cpus().first();

    let name = first
        .map(|c| Measurement::available(c.brand().to_string(), SYSINFO))
        .unwrap_or_else(|| unavail_str("CPU not detected."));
    let manufacturer = first
        .map(|c| Measurement::available(c.vendor_id().to_string(), SYSINFO))
        .unwrap_or_else(|| unavail_str("CPU vendor not detected."));
    let architecture = sysinfo::System::cpu_arch()
        .map(|v| Measurement::available(v, SYSINFO))
        .unwrap_or_else(|| unavail_str("Architecture unavailable."));
    let physical = sys
        .physical_core_count()
        .map(|v| Measurement::available(v as u32, SYSINFO))
        .unwrap_or_else(|| Measurement::unavailable(SYSINFO, "Physical core count unavailable."));
    let logical = Measurement::available(sys.cpus().len() as u32, SYSINFO);
    let current = first
        .map(|c| Measurement::available(c.frequency(), SYSINFO))
        .unwrap_or_else(|| Measurement::unavailable(SYSINFO, "Frequency unavailable."));
    let usage = Measurement::available(sys.global_cpu_usage(), SYSINFO);

    let base = wmi_cpu
        .as_ref()
        .and_then(|r| wmi::get_u64(r, "MaxClockSpeed"))
        .map(|v| Measurement::available(v, "WMI Win32_Processor"))
        .unwrap_or_else(|| {
            Measurement::unavailable("WMI Win32_Processor", "Base clock not exposed.")
        });
    let max = base.clone();
    let l2 = wmi_cpu
        .as_ref()
        .and_then(|r| wmi::get_u64(r, "L2CacheSize"))
        .map(|v| Measurement::available(v as u32, "WMI Win32_Processor"))
        .unwrap_or_else(|| {
            Measurement::unavailable("WMI Win32_Processor", "L2 cache not exposed.")
        });
    let l3 = wmi_cpu
        .as_ref()
        .and_then(|r| wmi::get_u64(r, "L3CacheSize"))
        .map(|v| Measurement::available(v as u32, "WMI Win32_Processor"))
        .unwrap_or_else(|| {
            Measurement::unavailable("WMI Win32_Processor", "L3 cache not exposed.")
        });
    let virt = match wmi_cpu
        .as_ref()
        .and_then(|r| wmi::get_bool(r, "VirtualizationFirmwareEnabled"))
    {
        Some(b) => Measurement::available(
            if b {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            },
            "WMI Win32_Processor",
        ),
        None => Measurement::unavailable("WMI Win32_Processor", "Not exposed."),
    };

    CpuInfo {
        name,
        manufacturer,
        architecture,
        physical_cores: physical,
        logical_processors: logical,
        base_frequency_mhz: base,
        current_frequency_mhz: current,
        max_frequency_mhz: max,
        usage_pct: usage,
        temperature_c: cpu_temperature(),
        virtualization: virt,
        l2_cache_kb: l2,
        l3_cache_kb: l3,
    }
}

/// CPU temperature from sysinfo Components, only if a CPU-labelled sensor exists.
/// Never inferred from usage.
#[cfg(windows)]
pub fn cpu_temperature() -> Measurement<f32> {
    let components = sysinfo::Components::new_with_refreshed_list();
    for c in components.list() {
        let label = c.label().to_lowercase();
        if label.contains("cpu")
            || label.contains("package")
            || label.contains("tdie")
            || label.contains("tctl")
        {
            let t = c.temperature();
            if t.is_finite() && t > 0.0 && t < 150.0 {
                return Measurement::new(
                    t,
                    Status::Available,
                    Confidence::Medium,
                    "sysinfo Components (thermal sensor)",
                );
            }
        }
    }
    Measurement::unsupported(
        "sysinfo Components",
        "No CPU temperature sensor exposed through a supported interface.",
    )
}

#[cfg(windows)]
pub fn gpu_info() -> Vec<GpuInfo> {
    use super::wmi;
    let rows = match wmi::cimv2("SELECT Name, AdapterCompatibility, DriverVersion, DriverDate, AdapterRAM, VideoModeDescription, CurrentRefreshRate FROM Win32_VideoController") {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    rows.into_iter()
        .map(|row| {
            let s = |key: &str| {
                wmi::get_str(&row, key)
                    .map(|v| Measurement::available(v, "WMI Win32_VideoController"))
                    .unwrap_or_else(|| {
                        Measurement::unavailable("WMI Win32_VideoController", "Not exposed.")
                    })
            };
            let dedicated = wmi::get_u64(&row, "AdapterRAM")
                .map(|v| Measurement::available(v, "WMI Win32_VideoController"))
                .unwrap_or_else(|| {
                    Measurement::unavailable(
                        "WMI Win32_VideoController",
                        "Adapter RAM not exposed.",
                    )
                });
            let refresh = wmi::get_u64(&row, "CurrentRefreshRate")
                .map(|v| Measurement::available(v as u32, "WMI Win32_VideoController"))
                .unwrap_or_else(|| {
                    Measurement::unavailable(
                        "WMI Win32_VideoController",
                        "Refresh rate not exposed.",
                    )
                });
            GpuInfo {
                name: s("Name"),
                manufacturer: s("AdapterCompatibility"),
                driver_version: s("DriverVersion"),
                driver_date: s("DriverDate"),
                dedicated_memory_bytes: dedicated,
                shared_memory_bytes: Measurement::unsupported(
                    "WMI Win32_VideoController",
                    "Shared memory not exposed by this interface.",
                ),
                current_resolution: s("VideoModeDescription"),
                refresh_rate_hz: refresh,
                usage_pct: Measurement::unsupported(
                    "GPU counters",
                    "GPU utilization requires a vendor/perf-counter interface not bundled here.",
                ),
                temperature_c: Measurement::unsupported(
                    "GPU sensor",
                    "GPU temperature requires a vendor-specific interface.",
                ),
            }
        })
        .collect()
}

#[cfg(windows)]
pub fn memory_info() -> MemoryInfo {
    use super::wmi;
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();
    let total = sys.total_memory();
    let used = sys.used_memory();
    let available = sys.available_memory();
    let usage = if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    };

    let modules = wmi::cimv2("SELECT Manufacturer, PartNumber, Capacity, Speed, SMBIOSMemoryType, DeviceLocator, BankLabel FROM Win32_PhysicalMemory")
        .map(|rows| rows.into_iter().map(|row| {
            let s = |key: &str| wmi::get_str(&row, key).map(|v| Measurement::available(v, "WMI Win32_PhysicalMemory")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_PhysicalMemory", "Not exposed."));
            let capacity = wmi::get_u64(&row, "Capacity").map(|v| Measurement::available(v, "WMI Win32_PhysicalMemory")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_PhysicalMemory", "Capacity not exposed."));
            let speed = wmi::get_u64(&row, "Speed").map(|v| Measurement::available(v as u32, "WMI Win32_PhysicalMemory")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_PhysicalMemory", "Speed not exposed."));
            let mtype = wmi::get_u64(&row, "SMBIOSMemoryType").map(|v| Measurement::available(wmi::memory_type_label(v), "WMI Win32_PhysicalMemory (SMBIOSMemoryType)")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_PhysicalMemory", "Memory type not exposed."));
            let slot = wmi::get_str(&row, "DeviceLocator").or_else(|| wmi::get_str(&row, "BankLabel")).map(|v| Measurement::available(v, "WMI Win32_PhysicalMemory")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_PhysicalMemory", "Slot not exposed."));
            MemoryModule { manufacturer: s("Manufacturer"), part_number: s("PartNumber"), capacity_bytes: capacity, speed_mhz: speed, memory_type: mtype, slot }
        }).collect())
        .unwrap_or_default();

    MemoryInfo {
        total_bytes: Measurement::available(total, SYSINFO),
        used_bytes: Measurement::available(used, SYSINFO),
        available_bytes: Measurement::available(available, SYSINFO),
        usage_pct: Measurement::available(usage, SYSINFO),
        modules,
    }
}

#[cfg(windows)]
fn format_epoch(secs: u64) -> String {
    vyronix_core::report::format_timestamp(secs * 1000)
}

// ============================ Non-Windows fallback ============================
// The desktop product targets Windows. On other platforms we return honest
// "unavailable" data so the code compiles and never fabricates hardware values.

#[cfg(not(windows))]
pub fn system_overview() -> SystemOverview {
    let u = || {
        Measurement::unavailable(
            "n/a",
            "Windows-only interface; build the Windows desktop app.",
        )
    };
    SystemOverview {
        device_name: u(),
        manufacturer: u(),
        model: u(),
        system_type: u(),
        motherboard_manufacturer: u(),
        motherboard_model: u(),
        bios_version: u(),
        bios_date: u(),
        os_edition: u(),
        os_version: u(),
        os_build: u(),
        os_arch: u(),
        install_date: u(),
        secure_boot: u(),
        tpm: u(),
        virtualization: u(),
        uptime_secs: Measurement::unavailable("n/a", "Windows-only"),
        boot_time: u(),
    }
}

#[cfg(not(windows))]
pub fn cpu_info() -> CpuInfo {
    let u = || Measurement::unavailable("n/a", "Windows-only interface.");
    CpuInfo {
        name: u(),
        manufacturer: u(),
        architecture: u(),
        physical_cores: Measurement::unavailable("n/a", "Windows-only"),
        logical_processors: Measurement::unavailable("n/a", "Windows-only"),
        base_frequency_mhz: Measurement::unavailable("n/a", "Windows-only"),
        current_frequency_mhz: Measurement::unavailable("n/a", "Windows-only"),
        max_frequency_mhz: Measurement::unavailable("n/a", "Windows-only"),
        usage_pct: Measurement::unavailable("n/a", "Windows-only"),
        temperature_c: Measurement::unavailable("n/a", "Windows-only"),
        virtualization: u(),
        l2_cache_kb: Measurement::unavailable("n/a", "Windows-only"),
        l3_cache_kb: Measurement::unavailable("n/a", "Windows-only"),
    }
}

#[cfg(not(windows))]
pub fn cpu_temperature() -> Measurement<f32> {
    Measurement::unavailable("n/a", "Windows-only interface.")
}

#[cfg(not(windows))]
pub fn gpu_info() -> Vec<GpuInfo> {
    Vec::new()
}

#[cfg(not(windows))]
pub fn memory_info() -> MemoryInfo {
    MemoryInfo {
        total_bytes: Measurement::unavailable("n/a", "Windows-only"),
        used_bytes: Measurement::unavailable("n/a", "Windows-only"),
        available_bytes: Measurement::unavailable("n/a", "Windows-only"),
        usage_pct: Measurement::unavailable("n/a", "Windows-only"),
        modules: Vec::new(),
    }
}
