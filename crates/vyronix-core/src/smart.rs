//! Storage S.M.A.R.T. interpretation.
//!
//! Two families are handled:
//!  * ATA SMART attributes (normalized value + worst + threshold + raw), and
//!  * the NVMe Health / Information log (512-byte structure).
//!
//! Interpretation is conservative and vendor-aware: critical ATA attributes
//! are judged on raw counts and the vendor's own threshold, never on a single
//! universal cutoff. NVMe uses the spec-defined fields.

use serde::{Deserialize, Serialize};

use crate::model::Severity;

/// A single ATA SMART attribute as returned by the device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtaAttribute {
    pub id: u8,
    pub name: String,
    /// Normalized current value (vendor scale, usually 0..=100 or 0..=200).
    pub current: u8,
    pub worst: u8,
    /// Vendor failure threshold for the normalized value (0 = no threshold).
    pub threshold: u8,
    /// Raw vendor value; meaning is attribute-specific.
    pub raw: u64,
}

/// Decoded NVMe health log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NvmeHealth {
    pub critical_warning: u8,
    pub temperature_c: Option<f64>,
    pub available_spare_pct: u8,
    pub available_spare_threshold_pct: u8,
    pub percentage_used: u8,
    pub data_units_read: u64,
    pub data_units_written: u64,
    pub host_read_commands: u64,
    pub host_write_commands: u64,
    pub controller_busy_time_min: u64,
    pub power_cycles: u64,
    pub power_on_hours: u64,
    pub unsafe_shutdowns: u64,
    pub media_errors: u64,
    pub error_log_entries: u64,
}

/// Parse the 512-byte NVMe Health Information log (NVMe spec figure 94).
/// All multi-byte fields are little-endian. Returns None if the buffer is too
/// short — never fabricates fields.
pub fn parse_nvme_health(buf: &[u8]) -> Option<NvmeHealth> {
    if buf.len() < 192 {
        return None;
    }
    let le16 = |o: usize| u16::from_le_bytes([buf[o], buf[o + 1]]) as u64;
    let le64 = |o: usize| {
        let mut b = [0u8; 8];
        b.copy_from_slice(&buf[o..o + 8]);
        u64::from_le_bytes(b)
    };
    let le128 = |o: usize| {
        let mut b = [0u8; 16];
        b.copy_from_slice(&buf[o..o + 16]);
        u128::from_le_bytes(b) as u64 // saturate; counters beyond u64 are not meaningful here
    };

    let kelvin = le16(1);
    let temperature_c = if kelvin == 0 {
        None
    } else {
        Some(kelvin as f64 - 273.15)
    };

    Some(NvmeHealth {
        critical_warning: buf[0],
        temperature_c,
        available_spare_pct: buf[3],
        available_spare_threshold_pct: buf[4],
        percentage_used: buf[5],
        data_units_read: le128(32),
        data_units_written: le128(48),
        host_read_commands: le128(64),
        host_write_commands: le128(80),
        controller_busy_time_min: le128(96),
        power_cycles: le128(112),
        power_on_hours: le128(128),
        unsafe_shutdowns: le128(144),
        media_errors: le128(160),
        error_log_entries: le128(176),
    })
}

/// Interpret the NVMe critical-warning byte into human-readable flags.
pub fn nvme_critical_flags(byte: u8) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if byte & 0x01 != 0 {
        flags.push("Available spare below threshold");
    }
    if byte & 0x02 != 0 {
        flags.push("Temperature above/below threshold");
    }
    if byte & 0x04 != 0 {
        flags.push("NVM subsystem reliability degraded");
    }
    if byte & 0x08 != 0 {
        flags.push("Media placed in read-only mode");
    }
    if byte & 0x10 != 0 {
        flags.push("Volatile memory backup device failed");
    }
    if byte & 0x20 != 0 {
        flags.push("Persistent memory region unreliable");
    }
    flags
}

/// Severity for an NVMe health log, per spec semantics.
pub fn nvme_severity(h: &NvmeHealth) -> Severity {
    if h.critical_warning != 0 {
        return Severity::Critical;
    }
    if h.available_spare_pct <= h.available_spare_threshold_pct {
        return Severity::Critical;
    }
    if h.media_errors > 0 {
        return Severity::Warning;
    }
    // percentage_used is an estimate of life consumed; >100 means past rated life.
    if h.percentage_used >= 100 {
        return Severity::Warning;
    }
    if h.percentage_used >= 90
        || h.available_spare_pct <= h.available_spare_threshold_pct.saturating_add(10)
    {
        return Severity::Information;
    }
    Severity::Good
}

/// Well-known ATA attribute IDs with safety-critical raw-count semantics.
pub mod ata_ids {
    pub const REALLOCATED_SECTOR: u8 = 5;
    pub const POWER_ON_HOURS: u8 = 9;
    pub const POWER_CYCLE_COUNT: u8 = 12;
    pub const SPIN_RETRY: u8 = 10;
    pub const REPORTED_UNCORRECTABLE: u8 = 187;
    pub const COMMAND_TIMEOUT: u8 = 188;
    pub const CURRENT_PENDING_SECTOR: u8 = 197;
    pub const OFFLINE_UNCORRECTABLE: u8 = 198;
    pub const UDMA_CRC_ERROR: u8 = 199;
    pub const TEMPERATURE: u8 = 194;
    pub const AIRFLOW_TEMPERATURE: u8 = 190;
}

/// Interpret a set of ATA attributes. Uses the vendor threshold for the
/// normalized value AND raw counts for the critical reallocation/pending/
/// uncorrectable family. Returns the worst severity found plus notes.
pub fn ata_severity(attrs: &[AtaAttribute]) -> (Severity, Vec<String>) {
    let mut worst = Severity::Good;
    let mut notes = Vec::new();
    let mut any = false;

    for a in attrs {
        any = true;
        // Vendor threshold breach on the normalized value (when defined).
        if a.threshold > 0 && a.current <= a.threshold {
            worst = Severity::Critical;
            notes.push(format!(
                "{} (id {}) normalized value {} ≤ vendor threshold {}",
                a.name, a.id, a.current, a.threshold
            ));
        }
        match a.id {
            ata_ids::REALLOCATED_SECTOR if a.raw > 0 => {
                worst = worst.max_by_rank(Severity::Warning);
                notes.push(format!("Reallocated sectors: {}", a.raw));
                if a.raw > 100 {
                    worst = Severity::Critical;
                }
            }
            ata_ids::CURRENT_PENDING_SECTOR if a.raw > 0 => {
                worst = Severity::Critical;
                notes.push(format!("Current pending sectors: {}", a.raw));
            }
            ata_ids::OFFLINE_UNCORRECTABLE if a.raw > 0 => {
                worst = Severity::Critical;
                notes.push(format!("Offline uncorrectable sectors: {}", a.raw));
            }
            ata_ids::SPIN_RETRY if a.raw > 0 => {
                worst = worst.max_by_rank(Severity::Warning);
                notes.push(format!("Spin retry count: {}", a.raw));
            }
            ata_ids::REPORTED_UNCORRECTABLE if a.raw > 0 => {
                worst = worst.max_by_rank(Severity::Warning);
                notes.push(format!("Reported uncorrectable errors: {}", a.raw));
            }
            ata_ids::UDMA_CRC_ERROR if a.raw > 0 => {
                worst = worst.max_by_rank(Severity::Information);
                notes.push(format!("UDMA CRC errors (often a cable issue): {}", a.raw));
            }
            _ => {}
        }
    }

    if !any {
        return (
            Severity::Unknown,
            vec!["No SMART attributes were available.".into()],
        );
    }
    (worst, notes)
}

impl Severity {
    /// Return the more severe of self and other (helper for accumulation).
    pub fn max_by_rank(self, other: Severity) -> Severity {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }
}

/// Extract temperature from ATA attributes when present (id 194 or 190).
/// The raw low byte is the current temperature for most drives.
pub fn ata_temperature(attrs: &[AtaAttribute]) -> Option<f64> {
    for a in attrs {
        if a.id == ata_ids::TEMPERATURE || a.id == ata_ids::AIRFLOW_TEMPERATURE {
            let t = (a.raw & 0xFF) as f64;
            if t > 0.0 && t < 150.0 {
                return Some(t);
            }
        }
    }
    None
}

/// Extract power-on hours from ATA attribute id 9 (raw hours for most drives).
pub fn ata_power_on_hours(attrs: &[AtaAttribute]) -> Option<u64> {
    attrs
        .iter()
        .find(|a| a.id == ata_ids::POWER_ON_HOURS)
        .map(|a| a.raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attr(id: u8, current: u8, worst: u8, threshold: u8, raw: u64) -> AtaAttribute {
        AtaAttribute {
            id,
            name: format!("attr_{id}"),
            current,
            worst,
            threshold,
            raw,
        }
    }

    #[test]
    fn nvme_parse_minimal() {
        let mut buf = vec![0u8; 512];
        buf[0] = 0; // no critical warning
        buf[1] = 0x3D;
        buf[2] = 0x01; // 317 K = 43.85 C
        buf[3] = 100; // available spare
        buf[4] = 10; // spare threshold
        buf[5] = 3; // percentage used
        buf[128] = 0xE8;
        buf[129] = 0x03; // 1000 power-on hours
        let h = parse_nvme_health(&buf).unwrap();
        assert_eq!(h.critical_warning, 0);
        assert!((h.temperature_c.unwrap() - 43.85).abs() < 0.01);
        assert_eq!(h.available_spare_pct, 100);
        assert_eq!(h.percentage_used, 3);
        assert_eq!(h.power_on_hours, 1000);
        assert_eq!(h.media_errors, 0);
    }

    #[test]
    fn nvme_short_buffer_returns_none() {
        assert!(parse_nvme_health(&[0u8; 10]).is_none());
    }

    #[test]
    fn nvme_zero_kelvin_is_none() {
        let buf = vec![0u8; 512];
        let h = parse_nvme_health(&buf).unwrap();
        assert!(h.temperature_c.is_none());
    }

    #[test]
    fn nvme_critical_warning_is_critical() {
        let mut buf = vec![0u8; 512];
        buf[0] = 0x04; // reliability degraded
        buf[3] = 100;
        buf[4] = 10;
        let h = parse_nvme_health(&buf).unwrap();
        assert_eq!(nvme_severity(&h), Severity::Critical);
        assert!(
            nvme_critical_flags(h.critical_warning).contains(&"NVM subsystem reliability degraded")
        );
    }

    #[test]
    fn nvme_spare_below_threshold_critical() {
        let mut buf = vec![0u8; 512];
        buf[3] = 5;
        buf[4] = 10;
        let h = parse_nvme_health(&buf).unwrap();
        assert_eq!(nvme_severity(&h), Severity::Critical);
    }

    #[test]
    fn nvme_healthy_is_good() {
        let mut buf = vec![0u8; 512];
        buf[3] = 100;
        buf[4] = 10;
        buf[5] = 1;
        let h = parse_nvme_health(&buf).unwrap();
        assert_eq!(nvme_severity(&h), Severity::Good);
    }

    #[test]
    fn nvme_media_errors_warning() {
        let mut buf = vec![0u8; 512];
        buf[3] = 100;
        buf[4] = 10;
        buf[5] = 1;
        buf[160] = 2; // media errors
        let h = parse_nvme_health(&buf).unwrap();
        assert_eq!(nvme_severity(&h), Severity::Warning);
    }

    #[test]
    fn ata_pending_sector_critical() {
        let attrs = vec![attr(197, 100, 100, 0, 3)];
        let (sev, notes) = ata_severity(&attrs);
        assert_eq!(sev, Severity::Critical);
        assert!(notes.iter().any(|n| n.contains("pending")));
    }

    #[test]
    fn ata_reallocated_warning_then_critical() {
        let (sev, _) = ata_severity(&[attr(5, 100, 100, 10, 5)]);
        assert_eq!(sev, Severity::Warning);
        let (sev2, _) = ata_severity(&[attr(5, 100, 100, 10, 500)]);
        assert_eq!(sev2, Severity::Critical);
    }

    #[test]
    fn ata_vendor_threshold_breach_critical() {
        let attrs = vec![attr(5, 5, 5, 10, 0)];
        let (sev, _) = ata_severity(&attrs);
        assert_eq!(sev, Severity::Critical);
    }

    #[test]
    fn ata_empty_is_unknown_not_good() {
        let (sev, _) = ata_severity(&[]);
        assert_eq!(sev, Severity::Unknown);
    }

    #[test]
    fn ata_temperature_extraction() {
        let attrs = vec![attr(194, 100, 100, 0, 38)];
        assert_eq!(ata_temperature(&attrs), Some(38.0));
        assert_eq!(ata_temperature(&[]), None);
    }

    #[test]
    fn ata_power_on_hours() {
        let attrs = vec![attr(9, 90, 90, 0, 12345)];
        assert_eq!(ata_power_on_hours(&attrs), Some(12345));
    }
}
