//! Battery collection via starship-battery (Windows battery IOCTL) + WMI.
//! Health/wear use the validated vyronix-core calculation — never fabricated.

use crate::model::BatteryInfo;
use vyronix_core::battery::{self, BatteryCapacities};
use vyronix_core::model::Measurement;

const SRC: &str = "Windows battery interface (starship-battery / IOCTL_BATTERY_QUERY_INFORMATION)";

pub fn battery_info() -> BatteryInfo {
    #[cfg(windows)]
    {
        windows_impl()
    }
    #[cfg(not(windows))]
    {
        empty(false)
    }
}

fn empty(present: bool) -> BatteryInfo {
    let u = || Measurement::unavailable("n/a", "No battery data available.");
    BatteryInfo {
        present,
        manufacturer: u(), model: u(), chemistry: u(),
        design_capacity_mwh: Measurement::unavailable("n/a", "No battery"),
        full_charge_capacity_mwh: Measurement::unavailable("n/a", "No battery"),
        remaining_capacity_mwh: Measurement::unavailable("n/a", "No battery"),
        design_voltage_mv: Measurement::unavailable("n/a", "No battery"),
        current_voltage_mv: Measurement::unavailable("n/a", "No battery"),
        charge_rate_mw: Measurement::unavailable("n/a", "No battery"),
        discharge_rate_mw: Measurement::unavailable("n/a", "No battery"),
        charging_status: u(), ac_connected: Measurement::unavailable("n/a", "No battery"),
        cycle_count: Measurement::unsupported("n/a", "No battery"),
        health_pct: Measurement::unavailable("n/a", "No battery"),
        wear_pct: Measurement::unavailable("n/a", "No battery"),
        state_of_charge_pct: Measurement::unavailable("n/a", "No battery"),
        estimated_runtime_hours: Measurement::unavailable("n/a", "No battery"),
        temperature_c: Measurement::unsupported("n/a", "No battery"),
    }
}

#[cfg(windows)]
fn windows_impl() -> BatteryInfo {
    use starship_battery::units::{electric_potential, energy, power, ratio, thermodynamic_temperature, time};
    use starship_battery::{Manager, State, Technology};

    let manager = match Manager::new() {
        Ok(m) => m,
        Err(_) => return empty(false),
    };
    let mut batteries = match manager.batteries() {
        Ok(b) => b,
        Err(_) => return empty(false),
    };
    let bat = match batteries.next() {
        Some(Ok(b)) => b,
        _ => return empty(false),
    };

    let design_mwh = bat.energy_full_design().get::<energy::milliwatt_hour>() as f64;
    let full_mwh = bat.energy_full().get::<energy::milliwatt_hour>() as f64;
    let remaining_mwh = bat.energy().get::<energy::milliwatt_hour>() as f64;
    let rate_mw = bat.energy_rate().get::<power::milliwatt>() as f64;
    let voltage_mv = bat.voltage().get::<electric_potential::millivolt>() as f64;
    let soc_pct = bat.state_of_charge().get::<ratio::percent>();

    let design_valid = design_mwh.is_finite() && design_mwh > 0.0;
    let full_valid = full_mwh.is_finite() && full_mwh > 0.0;
    let caps = BatteryCapacities {
        design_mwh: if design_valid { Some(design_mwh as u64) } else { None },
        full_charge_mwh: if full_valid { Some(full_mwh as u64) } else { None },
        remaining_mwh: if remaining_mwh.is_finite() && remaining_mwh >= 0.0 { Some(remaining_mwh as u64) } else { None },
    };
    let health = battery::health_measurement(&caps, SRC);
    let wear = match health.value {
        Some(h) => Measurement::new(100.0 - h, health.status, health.confidence, SRC)
            .with_note("Wear = 100 − health. Derived from the same validated capacities."),
        None => Measurement::unavailable(SRC, "Health not computable; wear cannot be derived."),
    };

    // Wildcard arm keeps this robust to the enum's hidden non-exhaustive variant.
    let (charging_status, ac_connected) = match bat.state() {
        State::Charging => ("Charging".to_string(), Some(true)),
        State::Discharging => ("Discharging".to_string(), Some(false)),
        State::Full => ("Full".to_string(), Some(true)),
        State::Empty => ("Empty".to_string(), Some(false)),
        _ => ("Unknown".to_string(), None),
    };

    let chemistry = match bat.technology() {
        Technology::LithiumIon => "Lithium-ion",
        Technology::LithiumPolymer => "Lithium-polymer",
        Technology::LithiumIronPhosphate => "LiFePO4",
        Technology::NickelMetalHydride => "NiMH",
        Technology::NickelCadmium => "NiCd",
        Technology::LeadAcid => "Lead-acid",
        _ => "Unknown",
    };

    // Discharge rate is positive when discharging; charge rate when charging.
    let discharging = bat.state() == State::Discharging;
    let charge_rate = if !discharging && rate_mw > 0.0 { rate_mw as u64 } else { 0 };
    let discharge_rate = if discharging && rate_mw > 0.0 { rate_mw as u64 } else { 0 };

    let runtime = battery::estimate_runtime_hours(caps.remaining_mwh, if discharge_rate > 0 { Some(discharge_rate) } else { None })
        .map(|h| Measurement::estimated(h, SRC, "Estimated: remaining capacity ÷ current discharge rate. Varies with load."))
        .unwrap_or_else(|| {
            // Fall back to the OS time-to-empty estimate if present.
            bat.time_to_empty()
                .map(|t| Measurement::estimated(t.get::<time::second>() as f64 / 3600.0, SRC, "OS-reported time-to-empty estimate."))
                .unwrap_or_else(|| Measurement::unavailable(SRC, "Not discharging or rate unavailable; runtime not estimated."))
        });

    let design_voltage = wmi_design_voltage_mv();

    BatteryInfo {
        present: true,
        manufacturer: bat.vendor().map(|v| Measurement::available(v.to_string(), SRC)).unwrap_or_else(|| Measurement::unavailable(SRC, "Not reported.")),
        model: bat.model().map(|v| Measurement::available(v.to_string(), SRC)).unwrap_or_else(|| Measurement::unavailable(SRC, "Not reported.")),
        chemistry: Measurement::available(chemistry.to_string(), SRC),
        design_capacity_mwh: if design_valid { Measurement::available(design_mwh as u64, SRC) } else { Measurement::unavailable(SRC, "Design capacity not reported.") },
        full_charge_capacity_mwh: if full_valid { Measurement::available(full_mwh as u64, SRC) } else { Measurement::unavailable(SRC, "Full-charge capacity not reported.") },
        remaining_capacity_mwh: if remaining_mwh.is_finite() && remaining_mwh >= 0.0 { Measurement::available(remaining_mwh as u64, SRC) } else { Measurement::unavailable(SRC, "Remaining capacity not reported.") },
        design_voltage_mv: design_voltage,
        current_voltage_mv: if voltage_mv.is_finite() && voltage_mv > 0.0 { Measurement::available(voltage_mv as u64, SRC) } else { Measurement::unavailable(SRC, "Voltage not reported.") },
        charge_rate_mw: Measurement::available(charge_rate, SRC),
        discharge_rate_mw: Measurement::available(discharge_rate, SRC),
        charging_status: Measurement::available(charging_status, SRC),
        ac_connected: match ac_connected {
            Some(b) => Measurement::available(b, SRC),
            None => Measurement::unavailable(SRC, "AC state could not be determined."),
        },
        cycle_count: bat.cycle_count().map(|c| Measurement::available(c, SRC)).unwrap_or_else(|| Measurement::unsupported(SRC, "Cycle count not reported by this battery's firmware.")),
        health_pct: health,
        wear_pct: wear,
        state_of_charge_pct: Measurement::available(soc_pct, SRC),
        estimated_runtime_hours: runtime,
        temperature_c: bat.temperature().map(|t| Measurement::available(t.get::<thermodynamic_temperature::degree_celsius>(), SRC)).unwrap_or_else(|| Measurement::unsupported(SRC, "Battery temperature not reported.")),
    }
}

#[cfg(windows)]
fn wmi_design_voltage_mv() -> Measurement<u64> {
    use super::wmi;
    match wmi::cimv2("SELECT DesignVoltage FROM Win32_Battery") {
        Ok(rows) => rows.into_iter().next()
            .and_then(|r| wmi::get_u64(&r, "DesignVoltage"))
            .filter(|v| *v > 0)
            .map(|v| Measurement::available(v, "WMI Win32_Battery"))
            .unwrap_or_else(|| Measurement::unavailable("WMI Win32_Battery", "Design voltage not reported.")),
        Err(_) => Measurement::unavailable("WMI Win32_Battery", "Battery WMI class unavailable."),
    }
}
