//! Windows WMI/CIM query helpers.
//!
//! Queries are read-only. Results are converted to JSON so callers can extract
//! fields defensively without type-mismatch panics. Every failure is surfaced
//! as an error string, never fabricated data.

#![cfg(windows)]

use std::collections::HashMap;
use wmi::{Variant, WMIConnection};

/// Convert a WMI Variant to a JSON value. Handles every variant explicitly.
fn variant_to_json(v: &Variant) -> serde_json::Value {
    use serde_json::json;
    match v {
        Variant::Empty | Variant::Null => serde_json::Value::Null,
        Variant::String(s) => json!(s),
        Variant::I1(n) => json!(n),
        Variant::I2(n) => json!(n),
        Variant::I4(n) => json!(n),
        Variant::I8(n) => json!(n),
        Variant::R4(n) => json!(n),
        Variant::R8(n) => json!(n),
        Variant::Bool(b) => json!(b),
        Variant::UI1(n) => json!(n),
        Variant::UI2(n) => json!(n),
        Variant::UI4(n) => json!(n),
        Variant::UI8(n) => json!(n),
        Variant::Array(arr) => serde_json::Value::Array(arr.iter().map(variant_to_json).collect()),
        Variant::Unknown(_) | Variant::Object(_) => serde_json::Value::Null,
    }
}

pub type WmiRow = serde_json::Map<String, serde_json::Value>;

/// Run a read-only WQL query in the given namespace (None = root\cimv2).
pub fn query_json(namespace: Option<&str>, query: &str) -> Result<Vec<WmiRow>, String> {
    let con = match namespace {
        Some(ns) => WMIConnection::with_namespace_path(ns),
        None => WMIConnection::new(),
    }
    .map_err(|e| format!("WMI connection failed: {e}"))?;

    let rows: Vec<HashMap<String, Variant>> = con
        .raw_query(query)
        .map_err(|e| format!("WMI query failed: {e}"))?;

    Ok(rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|(k, v)| (k, variant_to_json(&v)))
                .collect()
        })
        .collect())
}

/// Convenience: query root\cimv2.
pub fn cimv2(query: &str) -> Result<Vec<WmiRow>, String> {
    query_json(None, query)
}

// ---- Defensive field extractors ----

pub fn get_str(row: &WmiRow, key: &str) -> Option<String> {
    match row.get(key) {
        Some(serde_json::Value::String(s)) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Some(serde_json::Value::Number(n)) => Some(n.to_string()),
        _ => None,
    }
}

pub fn get_u64(row: &WmiRow, key: &str) -> Option<u64> {
    match row.get(key) {
        Some(serde_json::Value::Number(n)) => {
            n.as_u64().or_else(|| n.as_f64().map(|f| f.max(0.0) as u64))
        }
        Some(serde_json::Value::String(s)) => s.trim().parse::<u64>().ok(),
        _ => None,
    }
}

pub fn get_f64(row: &WmiRow, key: &str) -> Option<f64> {
    match row.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_f64(),
        Some(serde_json::Value::String(s)) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

pub fn get_bool(row: &WmiRow, key: &str) -> Option<bool> {
    match row.get(key) {
        Some(serde_json::Value::Bool(b)) => Some(*b),
        Some(serde_json::Value::Number(n)) => n.as_u64().map(|v| v != 0),
        _ => None,
    }
}

/// Map a WMI memory-type code (SMBIOSMemoryType) to a label.
pub fn memory_type_label(code: u64) -> String {
    match code {
        20 => "DDR".into(),
        21 => "DDR2".into(),
        22 => "DDR2 FB-DIMM".into(),
        24 => "DDR3".into(),
        26 => "DDR4".into(),
        34 => "DDR5".into(),
        other => format!("Unknown (code {other})"),
    }
}

/// Map a WMI ConfigManagerErrorCode to a short status.
pub fn device_error_status(code: u64) -> &'static str {
    match code {
        0 => "Working properly",
        1 => "Not configured",
        3 => "Driver may be corrupted",
        10 => "Device cannot start",
        12 => "Not enough resources",
        18 => "Reinstall driver",
        22 => "Device disabled",
        28 => "Drivers not installed",
        31 => "Not working properly",
        _ => "Problem reported",
    }
}
