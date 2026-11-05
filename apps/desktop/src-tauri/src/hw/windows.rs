//! Read-only Windows diagnostics: processes, services, drivers, startup items
//! and recent critical/error system events. No changes are made to the system.

use crate::model::*;

pub fn windows_diagnostics() -> WindowsDiagnostics {
    #[cfg(windows)]
    {
        WindowsDiagnostics {
            processes: processes(),
            services: services(),
            drivers: drivers(),
            startup: startup(),
            critical_events: critical_events(),
        }
    }
    #[cfg(not(windows))]
    {
        WindowsDiagnostics { processes: Vec::new(), services: Vec::new(), drivers: Vec::new(), startup: Vec::new(), critical_events: Vec::new() }
    }
}

#[cfg(windows)]
fn processes() -> Vec<ProcessEntry> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_cpu_usage();
    // A second refresh after a short interval yields a meaningful CPU delta.
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_usage();
    let mut entries: Vec<ProcessEntry> = sys.processes().iter().map(|(pid, p)| ProcessEntry {
        pid: pid.as_u32(),
        name: p.name().to_string_lossy().to_string(),
        cpu_pct: p.cpu_usage(),
        memory_bytes: p.memory(),
    }).collect();
    entries.sort_by(|a, b| b.cpu_pct.partial_cmp(&a.cpu_pct).unwrap_or(std::cmp::Ordering::Equal));
    entries.truncate(300);
    entries
}

#[cfg(windows)]
fn services() -> Vec<ServiceEntry> {
    use super::wmi;
    wmi::cimv2("SELECT Name, DisplayName, State, StartMode FROM Win32_Service")
        .map(|rows| rows.into_iter().take(300).map(|r| ServiceEntry {
            name: wmi::get_str(&r, "Name").unwrap_or_default(),
            display_name: wmi::get_str(&r, "DisplayName").unwrap_or_default(),
            state: wmi::get_str(&r, "State").unwrap_or_else(|| "Unknown".into()),
            start_mode: wmi::get_str(&r, "StartMode").unwrap_or_else(|| "Unknown".into()),
        }).collect())
        .unwrap_or_default()
}

#[cfg(windows)]
fn drivers() -> Vec<DriverEntry> {
    use super::wmi;
    use std::collections::HashMap;
    // Device health / error codes from PnP entities.
    let mut err_by_id: HashMap<String, u64> = HashMap::new();
    if let Ok(rows) = wmi::cimv2("SELECT DeviceID, ConfigManagerErrorCode FROM Win32_PnPEntity") {
        for r in rows {
            if let (Some(id), Some(code)) = (wmi::get_str(&r, "DeviceID"), wmi::get_u64(&r, "ConfigManagerErrorCode")) {
                err_by_id.insert(id, code);
            }
        }
    }
    let mut entries: Vec<DriverEntry> = wmi::cimv2("SELECT DeviceName, DriverVersion, DriverDate, DeviceID FROM Win32_PnPSignedDriver")
        .map(|rows| rows.into_iter().map(|r| {
            let id = wmi::get_str(&r, "DeviceID").unwrap_or_default();
            let code = err_by_id.get(&id).copied().unwrap_or(0);
            DriverEntry {
                name: wmi::get_str(&r, "DeviceName").unwrap_or_else(|| "Unknown device".into()),
                version: wmi::get_str(&r, "DriverVersion").unwrap_or_else(|| "—".into()),
                date: wmi::get_str(&r, "DriverDate").unwrap_or_else(|| "—".into()),
                status: wmi::device_error_status(code).to_string(),
                error_code: code as u32,
            }
        }).collect())
        .unwrap_or_default();
    // Surface problem devices first.
    entries.sort_by(|a, b| b.error_code.cmp(&a.error_code));
    entries.truncate(300);
    entries
}

#[cfg(windows)]
fn startup() -> Vec<StartupEntry> {
    use super::wmi;
    wmi::cimv2("SELECT Name, Command, Location FROM Win32_StartupCommand")
        .map(|rows| rows.into_iter().take(100).map(|r| StartupEntry {
            name: wmi::get_str(&r, "Name").unwrap_or_default(),
            command: wmi::get_str(&r, "Command").unwrap_or_default(),
            location: wmi::get_str(&r, "Location").unwrap_or_default(),
        }).collect())
        .unwrap_or_default()
}

#[cfg(windows)]
fn critical_events() -> Vec<EventEntry> {
    // Read-only event log query via PowerShell Get-WinEvent (justified: the
    // Windows event log has no simple WMI surface for recent critical events).
    let script = "Get-WinEvent -FilterHashtable @{LogName='System'; Level=1,2} -MaxEvents 100 -ErrorAction SilentlyContinue | Select-Object TimeCreated, ProviderName, Id, @{n='Message';e={($_.Message -split \"`n\")[0]}} | ConvertTo-Json -Compress";
    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output();
    let text = match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => return Vec::new(),
    };
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    let value: serde_json::Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let arr = match value {
        serde_json::Value::Array(a) => a,
        obj @ serde_json::Value::Object(_) => vec![obj],
        _ => return Vec::new(),
    };
    arr.into_iter().take(100).map(|e| EventEntry {
        time: e.get("TimeCreated").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        source: e.get("ProviderName").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        event_id: e.get("Id").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        message: e.get("Message").and_then(|v| v.as_str()).unwrap_or("").chars().take(300).collect(),
    }).collect()
}
