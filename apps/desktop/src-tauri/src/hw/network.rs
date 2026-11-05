//! Network adapter collection + ICMP ping (Windows IcmpSendEcho, no admin).

use crate::model::{NetworkAdapter, PingResult};
use vyronix_core::model::Measurement;

const WMI: &str = "WMI Win32_NetworkAdapter / Configuration";

pub fn network_adapters() -> Vec<NetworkAdapter> {
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
fn join_array(row: &super::wmi::WmiRow, key: &str) -> Option<String> {
    match row.get(key) {
        Some(serde_json::Value::Array(arr)) => {
            let parts: Vec<String> = arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(", "))
            }
        }
        Some(serde_json::Value::String(s)) if !s.is_empty() => Some(s.clone()),
        _ => None,
    }
}

#[cfg(windows)]
fn windows_impl() -> Vec<NetworkAdapter> {
    use super::wmi;
    use std::collections::HashMap;

    let adapters = match wmi::cimv2("SELECT Index, Name, AdapterType, MACAddress, Speed, NetConnectionStatus FROM Win32_NetworkAdapter WHERE PhysicalAdapter=True") {
        Ok(a) => a,
        Err(_) => return Vec::new(),
    };
    let configs = wmi::cimv2("SELECT Index, IPAddress, DNSServerSearchOrder FROM Win32_NetworkAdapterConfiguration WHERE IPEnabled=True").unwrap_or_default();
    let cfg_by_index: HashMap<u64, wmi::WmiRow> = configs
        .into_iter()
        .filter_map(|r| wmi::get_u64(&r, "Index").map(|i| (i, r)))
        .collect();

    adapters
        .into_iter()
        .map(|a| {
            let index = wmi::get_u64(&a, "Index");
            let cfg = index.and_then(|i| cfg_by_index.get(&i));
            let s = |key: &str| {
                wmi::get_str(&a, key)
                    .map(|v| Measurement::available(v, WMI))
                    .unwrap_or_else(|| Measurement::unavailable(WMI, "Not reported."))
            };

            let speed = wmi::get_u64(&a, "Speed")
                .filter(|v| *v > 0)
                .map(|v| Measurement::available(v / 1_000_000, WMI))
                .unwrap_or_else(|| Measurement::unavailable(WMI, "Link speed not reported."));
            let connected = wmi::get_u64(&a, "NetConnectionStatus")
                .map(|v| Measurement::available(v == 2, WMI))
                .unwrap_or_else(|| {
                    Measurement::unavailable(WMI, "Connection status not reported.")
                });
            let ips = cfg
                .and_then(|c| join_array(c, "IPAddress"))
                .map(|v| Measurement::available(v, WMI))
                .unwrap_or_else(|| Measurement::unavailable(WMI, "No IP configuration."));
            let dns = cfg
                .and_then(|c| join_array(c, "DNSServerSearchOrder"))
                .map(|v| Measurement::available(v, WMI))
                .unwrap_or_else(|| Measurement::unavailable(WMI, "No DNS servers configured."));

            NetworkAdapter {
                name: s("Name"),
                adapter_type: s("AdapterType"),
                mac_address: s("MACAddress"),
                ip_addresses: ips,
                link_speed_mbps: speed,
                connected,
                dns_servers: dns,
                signal_pct: Measurement::unsupported(
                    "WLAN API",
                    "Wi-Fi signal strength requires the WLAN API; not exposed by WMI.",
                ),
            }
        })
        .collect()
}

/// ICMP echo using the Windows IP Helper API. Read-only network operation.
pub fn ping(host: &str, count: u32) -> Result<PingResult, String> {
    #[cfg(windows)]
    {
        windows_ping(host, count)
    }
    #[cfg(not(windows))]
    {
        let _ = (host, count);
        Err("ICMP ping uses the Windows IP Helper API; build the Windows desktop app.".into())
    }
}

#[cfg(windows)]
fn windows_ping(host: &str, count: u32) -> Result<PingResult, String> {
    use std::ffi::CString;
    use windows::core::PCSTR;
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::NetworkManagement::IpHelper::{
        IcmpCloseHandle, IcmpCreateFile, IcmpSendEcho, ICMP_ECHO_REPLY,
    };
    use windows::Win32::Networking::WinSock::inet_addr;

    let count = count.clamp(1, 20);
    let c_host = CString::new(host).map_err(|_| "Invalid host string")?;
    let dest = unsafe { inet_addr(PCSTR(c_host.as_ptr() as *const u8)) };
    if dest == u32::MAX {
        return Err(format!("Could not resolve '{host}' to an IPv4 address."));
    }

    let handle: HANDLE =
        unsafe { IcmpCreateFile() }.map_err(|e| format!("Failed to open ICMP handle: {e}"))?;

    let payload = [0x76u8; 32]; // "v" padding, standard 32-byte echo payload
    let reply_size = (std::mem::size_of::<ICMP_ECHO_REPLY>() + payload.len() + 8) as u32;
    let mut rtts: Vec<f64> = Vec::new();
    let mut received = 0u32;

    for _ in 0..count {
        let mut reply_buf = vec![0u8; reply_size as usize];
        let ret = unsafe {
            IcmpSendEcho(
                handle,
                dest,
                payload.as_ptr() as *const std::ffi::c_void,
                payload.len() as u16,
                None,
                reply_buf.as_mut_ptr() as *mut std::ffi::c_void,
                reply_size,
                2000,
            )
        };
        if ret > 0 {
            let reply = unsafe { &*(reply_buf.as_ptr() as *const ICMP_ECHO_REPLY) };
            if reply.Status == 0 {
                received += 1;
                rtts.push(reply.RoundTripTime as f64);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    unsafe {
        let _ = IcmpCloseHandle(handle);
    };

    let transmitted = count;
    let loss_pct = if transmitted > 0 {
        ((transmitted - received) as f64 / transmitted as f64) * 100.0
    } else {
        0.0
    };
    let (min, avg, max) = if rtts.is_empty() {
        (None, None, None)
    } else {
        let min = rtts.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = rtts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg = rtts.iter().sum::<f64>() / rtts.len() as f64;
        (Some(min), Some(avg), Some(max))
    };

    Ok(PingResult {
        host: host.to_string(),
        transmitted,
        received,
        loss_pct,
        min_ms: min,
        avg_ms: avg,
        max_ms: max,
    })
}
