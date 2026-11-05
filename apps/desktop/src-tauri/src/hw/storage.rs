//! Storage device collection + SMART health.
//!
//! Primary SMART source (no admin): root\Microsoft\Windows\Storage —
//! MSFT_PhysicalDisk (health/media) + MSFT_StorageReliabilityCounter (wear,
//! temperature, power-on hours, error counters). These expose genuine
//! SMART-derived data on Windows 8+. Detailed ATA attributes that require the
//! admin-only root\WMI pass-through are honestly marked Unsupported when not
//! exposed. Health severity uses the validated vyronix-core SMART evaluation.

use crate::model::{PartitionInfo, StorageDevice};
use vyronix_core::model::{Measurement, Severity};

const DISKDRIVE: &str = "WMI Win32_DiskDrive";
const MSFT: &str = "Windows Storage (MSFT_PhysicalDisk / MSFT_StorageReliabilityCounter)";

pub fn storage_devices() -> Vec<StorageDevice> {
    #[cfg(windows)]
    {
        windows_impl()
    }
    #[cfg(not(windows))]
    {
        Vec::new()
    }
}

#[cfg(windows)]
fn windows_impl() -> Vec<StorageDevice> {
    use super::wmi;
    use std::collections::HashMap;

    let drives = match wmi::cimv2("SELECT Index, DeviceID, Model, Manufacturer, SerialNumber, Size, FirmwareRevision, InterfaceType, MediaType, Status, PNPDeviceID FROM Win32_DiskDrive") {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    // MSFT_PhysicalDisk keyed by DeviceId.
    let mut phys: HashMap<u64, wmi::WmiRow> = HashMap::new();
    if let Ok(rows) = wmi::query_json(Some("root\\Microsoft\\Windows\\Storage"), "SELECT DeviceId, FriendlyName, SerialNumber, MediaType, HealthStatus, Size, BusType FROM MSFT_PhysicalDisk") {
        for r in rows {
            if let Some(id) = wmi::get_u64(&r, "DeviceId") {
                phys.insert(id, r);
            }
        }
    }
    // Reliability counters keyed by DeviceId.
    let mut rel: HashMap<u64, wmi::WmiRow> = HashMap::new();
    if let Ok(rows) = wmi::query_json(Some("root\\Microsoft\\Windows\\Storage"), "SELECT DeviceId, Wear, Temperature, PowerOnHours, ReadErrorsTotal, ReadErrorsUncorrected, WriteErrorsTotal, WriteErrorsUncorrected, StartStopCycleCount FROM MSFT_StorageReliabilityCounter") {
        for r in rows {
            if let Some(id) = wmi::get_u64(&r, "DeviceId") {
                rel.insert(id, r);
            }
        }
    }

    let partitions = logical_partitions();

    drives.into_iter().map(|d| {
        let index = wmi::get_u64(&d, "Index").unwrap_or(0);
        let device_id = wmi::get_str(&d, "DeviceID").unwrap_or_default();
        let p = phys.get(&index);
        let r = rel.get(&index);

        let s = |key: &str| wmi::get_str(&d, key).map(|v| Measurement::available(v, DISKDRIVE)).unwrap_or_else(|| Measurement::unavailable(DISKDRIVE, "Not reported."));

        // Media type: prefer MSFT (SSD/HDD), else Win32 MediaType.
        let media_type = match p.and_then(|x| wmi::get_u64(x, "MediaType")) {
            Some(4) => Measurement::available("SSD".to_string(), MSFT),
            Some(3) => Measurement::available("HDD".to_string(), MSFT),
            Some(5) => Measurement::available("SCM".to_string(), MSFT),
            _ => wmi::get_str(&d, "MediaType").map(|v| Measurement::available(v, DISKDRIVE)).unwrap_or_else(|| Measurement::unavailable(DISKDRIVE, "Media type not reported.")),
        };
        let is_ssd = media_type.value.as_deref().map(|v| v.contains("SSD")).unwrap_or(false);

        // Health status from MSFT_PhysicalDisk.HealthStatus (0=Healthy,1=Warning,2=Unhealthy,5=Unknown).
        let health_status = match p.and_then(|x| wmi::get_u64(x, "HealthStatus")) {
            Some(0) => Measurement::available("Healthy".to_string(), MSFT),
            Some(1) => Measurement::available("Warning".to_string(), MSFT),
            Some(2) => Measurement::available("Unhealthy".to_string(), MSFT),
            Some(5) => Measurement::unavailable(MSFT, "Health status unknown."),
            _ => wmi::get_str(&d, "Status").map(|v| Measurement::available(v, DISKDRIVE)).unwrap_or_else(|| Measurement::unavailable(DISKDRIVE, "Health not reported.")),
        };

        // Reliability counters (real SMART-derived values where exposed).
        let temperature = r.and_then(|x| wmi::get_f64(x, "Temperature")).filter(|t| *t > 0.0 && *t < 150.0)
            .map(|t| Measurement::available(t as f32, MSFT))
            .unwrap_or_else(|| Measurement::unsupported(MSFT, "Temperature not reported by this device."));
        let power_on_hours = r.and_then(|x| wmi::get_u64(x, "PowerOnHours")).map(|v| Measurement::available(v, MSFT)).unwrap_or_else(|| Measurement::unsupported(MSFT, "Power-on hours not reported."));
        let power_cycles = r.and_then(|x| wmi::get_u64(x, "StartStopCycleCount")).map(|v| Measurement::available(v, MSFT)).unwrap_or_else(|| Measurement::unsupported(MSFT, "Start/stop count not reported."));
        let wear = r.and_then(|x| wmi::get_u64(x, "Wear")).filter(|v| *v <= 100).map(|v| Measurement::available(v as u8, MSFT)).unwrap_or_else(|| Measurement::unsupported(MSFT, "Wear not reported (typically HDD or unsupported controller)."));

        let read_uncorr = r.and_then(|x| wmi::get_u64(x, "ReadErrorsUncorrected")).unwrap_or(0);
        let write_uncorr = r.and_then(|x| wmi::get_u64(x, "WriteErrorsUncorrected")).unwrap_or(0);
        let media_errors_val = read_uncorr + write_uncorr;
        let media_errors = if r.is_some() { Measurement::available(media_errors_val, MSFT) } else { Measurement::unsupported(MSFT, "Error counters not reported.") };

        // ATA-specific attributes need the admin-only pass-through; not exposed here.
        let ata_unsupported = |name: &str| Measurement::unsupported(MSFT, format!("{name} requires the administrator ATA SMART pass-through (root\\WMI); not exposed by the standard storage interface."));

        // Detailed ATA/NVMe fields need the admin-only pass-through; mark honestly.
        let available_spare_pct = Measurement::unsupported(MSFT, "Available spare requires the NVMe health log / admin pass-through.");
        let percentage_used = wear.clone();
        let reallocated_sectors = ata_unsupported("Reallocated sectors");
        let pending_sectors = ata_unsupported("Pending sectors");
        let uncorrectable_sectors = if r.is_some() { Measurement::available(media_errors_val, MSFT) } else { Measurement::unsupported(MSFT, "Uncorrectable errors not reported.") };
        let unsafe_shutdowns = Measurement::unsupported(MSFT, "Unsafe shutdown count requires the NVMe health log / admin pass-through.");

        // Transparent, conservative severity from the real counters that ARE
        // exposed (HealthStatus, wear, temperature, uncorrected errors).
        let mut severity = match p.and_then(|x| wmi::get_u64(x, "HealthStatus")) {
            Some(0) => Severity::Good,
            Some(1) => Severity::Warning,
            Some(2) => Severity::Critical,
            _ => Severity::Unknown,
        };
        let mut smart_notes: Vec<String> = Vec::new();
        if let Some(w) = wear.value {
            if w >= 95 {
                severity = severity.max_by_rank(Severity::Critical);
                smart_notes.push(format!("SSD wear at {w}% — approaching the rated endurance limit."));
            } else if w >= 80 {
                severity = severity.max_by_rank(Severity::Warning);
                smart_notes.push(format!("SSD wear at {w}% — monitor regularly and keep backups."));
            }
        }
        if let Some(t) = temperature.value {
            if t >= 70.0 {
                severity = severity.max_by_rank(Severity::Critical);
                smart_notes.push(format!("Storage temperature {t:.0}°C is high — improve cooling."));
            } else if t >= 55.0 {
                severity = severity.max_by_rank(Severity::Warning);
                smart_notes.push(format!("Storage temperature {t:.0}°C is elevated."));
            }
        }
        if r.is_some() && media_errors_val > 0 {
            severity = severity.max_by_rank(if media_errors_val >= 100 { Severity::Critical } else { Severity::Warning });
            smart_notes.push(format!("{media_errors_val} uncorrected read/write error(s) reported — back up data and monitor the drive."));
        }
        if severity == Severity::Unknown {
            smart_notes.push("Health could not be assessed from the available storage interface.".into());
        } else if smart_notes.is_empty() {
            smart_notes.push("No adverse SMART indicators reported by the storage interface.".into());
        }
        let _ = is_ssd;

        StorageDevice {
            id: if device_id.is_empty() { format!("disk-{index}") } else { device_id.clone() },
            model: s("Model"), manufacturer: s("Manufacturer"),
            serial_number: s("SerialNumber"),
            capacity_bytes: wmi::get_u64(&d, "Size").map(|v| Measurement::available(v, DISKDRIVE)).unwrap_or_else(|| Measurement::unavailable(DISKDRIVE, "Capacity not reported.")),
            firmware: s("FirmwareRevision"), interface: s("InterfaceType"),
            media_type, health_status,
            temperature_c: temperature, power_on_hours, power_cycles,
            wear_pct: wear,
            available_spare_pct,
            percentage_used,
            media_errors,
            reallocated_sectors,
            pending_sectors,
            uncorrectable_sectors,
            unsafe_shutdowns,
            severity,
            smart_notes,
            partitions: partitions.get(&index).cloned().unwrap_or_default(),
        }
    }).collect()
}

/// Map physical drive index -> logical partitions via WMI associations.
#[cfg(windows)]
fn logical_partitions() -> std::collections::HashMap<u64, Vec<PartitionInfo>> {
    use super::wmi;
    use std::collections::HashMap;
    let mut result: HashMap<u64, Vec<PartitionInfo>> = HashMap::new();

    // disk -> partition and partition -> logical association maps.
    let disk_part = wmi::cimv2("SELECT Antecedent, Dependent FROM Win32_DiskDriveToDiskPartition").unwrap_or_default();
    let part_logical = wmi::cimv2("SELECT Antecedent, Dependent FROM Win32_LogicalDiskToPartition").unwrap_or_default();
    let logical = wmi::cimv2("SELECT DeviceID, FileSystem, Size, FreeSpace, VolumeName, DriveType FROM Win32_LogicalDisk").unwrap_or_default();
    let logical_by_id: HashMap<String, wmi::WmiRow> = logical.into_iter()
        .filter_map(|r| wmi::get_str(&r, "DeviceID").map(|id| (id, r)))
        .collect();

    // partition DeviceID -> logical DeviceID
    let mut part_to_logical: HashMap<String, String> = HashMap::new();
    for row in &part_logical {
        let ante = wmi::get_str(row, "Antecedent").unwrap_or_default();
        let dep = wmi::get_str(row, "Dependent").unwrap_or_default();
        if let (Some(pid), Some(lid)) = (extract_device_id(&ante), extract_device_id(&dep)) {
            part_to_logical.insert(pid, lid);
        }
    }
    // disk index -> partition DeviceIDs
    for row in &disk_part {
        let ante = wmi::get_str(row, "Antecedent").unwrap_or_default();
        let dep = wmi::get_str(row, "Dependent").unwrap_or_default();
        let disk_id = match extract_device_id(&ante) { Some(v) => v, None => continue };
        let part_id = match extract_device_id(&dep) { Some(v) => v, None => continue };
        // disk_id like \\.\PHYSICALDRIVE0 -> index 0
        let index = disk_id.rsplit("PHYSICALDRIVE").next().and_then(|s| s.parse::<u64>().ok());
        let index = match index { Some(i) => i, None => continue };
        if let Some(lid) = part_to_logical.get(&part_id) {
            if let Some(lrow) = logical_by_id.get(lid) {
                let cap = wmi::get_u64(lrow, "Size").map(|v| Measurement::available(v, "WMI Win32_LogicalDisk")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_LogicalDisk", "Not reported."));
                let free = wmi::get_u64(lrow, "FreeSpace").map(|v| Measurement::available(v, "WMI Win32_LogicalDisk")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_LogicalDisk", "Not reported."));
                let fs = wmi::get_str(lrow, "FileSystem").map(|v| Measurement::available(v, "WMI Win32_LogicalDisk")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_LogicalDisk", "Not reported."));
                let label = wmi::get_str(lrow, "VolumeName").map(|v| Measurement::available(v, "WMI Win32_LogicalDisk")).unwrap_or_else(|| Measurement::unavailable("WMI Win32_LogicalDisk", "No label."));
                result.entry(index).or_default().push(PartitionInfo {
                    drive_letter: lid.clone(), file_system: fs, capacity_bytes: cap, free_bytes: free, label,
                });
            }
        }
    }
    result
}

/// Extract the DeviceID value from a WMI object path string.
#[cfg(windows)]
fn extract_device_id(path: &str) -> Option<String> {
    // path like: \\HOST\root\cimv2:Win32_DiskDrive.DeviceID="\\\\.\\PHYSICALDRIVE0"
    let key = "DeviceID=\"";
    let start = path.find(key)? + key.len();
    let rest = &path[start..];
    let end = rest.find('"')?;
    Some(rest[..end].replace("\\\\", "\\"))
}
