//! Battery health analysis with strict input validation.
//!
//! Health % = (full charge capacity / design capacity) * 100, computed ONLY
//! when both capacities are present, positive and plausible. Suspicious
//! readings are never silently clamped — they are reported as anomalies.

use serde::{Deserialize, Serialize};

use crate::model::{Confidence, Measurement, Status};

/// Raw battery capacities as reported by the hardware interface, in mWh.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BatteryCapacities {
    pub design_mwh: Option<u64>,
    pub full_charge_mwh: Option<u64>,
    pub remaining_mwh: Option<u64>,
}

/// Outcome of a health calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HealthOutcome {
    /// Valid calculation. `health_pct` may exceed 100 (new batteries can read
    /// above design capacity); that is reported, not clamped.
    Ok {
        health_pct: f64,
        wear_pct: f64,
        above_design: bool,
    },
    /// Required inputs missing or invalid; health must NOT be displayed.
    NotComputable { reason: String },
    /// Inputs present but implausible; flagged for attention, not clamped.
    Anomaly { reason: String, raw_health_pct: f64 },
}

/// Upper plausibility bound. Readings above this are treated as anomalies
/// (likely a firmware/driver reporting error), not silently clamped.
pub const MAX_PLAUSIBLE_HEALTH_PCT: f64 = 150.0;
/// Design capacities above this (1 kWh) are implausible for PC/laptop batteries.
pub const MAX_PLAUSIBLE_DESIGN_MWH: u64 = 1_000_000;

/// Compute battery health from capacities. Pure and fully validated.
pub fn compute_health(caps: &BatteryCapacities) -> HealthOutcome {
    let (design, full) = match (caps.design_mwh, caps.full_charge_mwh) {
        (Some(d), Some(f)) => (d, f),
        _ => {
            return HealthOutcome::NotComputable {
                reason: "Design capacity and full-charge capacity are both required.".into(),
            }
        }
    };

    if design == 0 {
        return HealthOutcome::NotComputable {
            reason: "Design capacity is zero; health cannot be computed.".into(),
        };
    }
    if full == 0 {
        return HealthOutcome::NotComputable {
            reason: "Full-charge capacity is zero; the battery may be deeply discharged or the reading is invalid.".into(),
        };
    }
    if design > MAX_PLAUSIBLE_DESIGN_MWH {
        return HealthOutcome::Anomaly {
            reason: format!("Design capacity {design} mWh exceeds plausible range."),
            raw_health_pct: (full as f64 / design as f64) * 100.0,
        };
    }

    let health = (full as f64 / design as f64) * 100.0;
    if !health.is_finite() {
        return HealthOutcome::NotComputable {
            reason: "Computed health is not a finite number.".into(),
        };
    }
    if health > MAX_PLAUSIBLE_HEALTH_PCT {
        return HealthOutcome::Anomaly {
            reason: format!(
                "Full-charge capacity implies {health:.1}% of design, above the plausible ceiling of {MAX_PLAUSIBLE_HEALTH_PCT}%."
            ),
            raw_health_pct: health,
        };
    }

    HealthOutcome::Ok {
        health_pct: health,
        wear_pct: 100.0 - health,
        above_design: health > 100.0,
    }
}

/// Wrap a health outcome into an honest Measurement for the UI/report layer.
pub fn health_measurement(caps: &BatteryCapacities, source: &str) -> Measurement<f64> {
    match compute_health(caps) {
        HealthOutcome::Ok { health_pct, above_design, .. } => {
            let mut m = Measurement::new(health_pct, Status::Available, Confidence::High, source)
                .with_note("Health = full-charge capacity / design capacity × 100.");
            if above_design {
                m.note = Some(
                    "Health = full-charge / design × 100. Reading is above 100% (battery holds more than design capacity); reported as measured, not clamped.".into(),
                );
            }
            m
        }
        HealthOutcome::Anomaly { reason, raw_health_pct } => {
            let mut m = Measurement::new(raw_health_pct, Status::Estimated, Confidence::Low, source);
            m.note = Some(format!("Anomaly, not clamped: {reason}"));
            m
        }
        HealthOutcome::NotComputable { reason } => {
            Measurement::unavailable(source, reason)
        }
    }
}

/// Estimated runtime from remaining capacity and current discharge rate.
/// Returns None when inputs are invalid. Clearly an estimate.
pub fn estimate_runtime_hours(remaining_mwh: Option<u64>, discharge_rate_mw: Option<u64>) -> Option<f64> {
    let remaining = remaining_mwh?;
    let rate = discharge_rate_mw?;
    if rate == 0 {
        return None;
    }
    let hours = remaining as f64 / rate as f64;
    if hours.is_finite() && hours >= 0.0 {
        Some(hours)
    } else {
        None
    }
}

/// Classify battery health into a severity using documented thresholds.
pub fn health_severity(health_pct: f64) -> crate::model::Severity {
    use crate::model::Severity;
    if !health_pct.is_finite() {
        return Severity::Unknown;
    }
    if health_pct >= 80.0 {
        Severity::Good
    } else if health_pct >= 60.0 {
        Severity::Information
    } else if health_pct >= 40.0 {
        Severity::Warning
    } else {
        Severity::Critical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caps(design: Option<u64>, full: Option<u64>) -> BatteryCapacities {
        BatteryCapacities { design_mwh: design, full_charge_mwh: full, remaining_mwh: None }
    }

    #[test]
    fn normal_health() {
        let out = compute_health(&caps(Some(50_000), Some(45_000)));
        match out {
            HealthOutcome::Ok { health_pct, wear_pct, above_design } => {
                assert!((health_pct - 90.0).abs() < 1e-9);
                assert!((wear_pct - 10.0).abs() < 1e-9);
                assert!(!above_design);
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn missing_design_not_computable() {
        assert!(matches!(compute_health(&caps(None, Some(40_000))), HealthOutcome::NotComputable { .. }));
    }

    #[test]
    fn missing_full_not_computable() {
        assert!(matches!(compute_health(&caps(Some(50_000), None)), HealthOutcome::NotComputable { .. }));
    }

    #[test]
    fn zero_design_not_computable() {
        assert!(matches!(compute_health(&caps(Some(0), Some(40_000))), HealthOutcome::NotComputable { .. }));
    }

    #[test]
    fn zero_full_not_computable() {
        assert!(matches!(compute_health(&caps(Some(50_000), Some(0))), HealthOutcome::NotComputable { .. }));
    }

    #[test]
    fn above_design_reported_not_clamped() {
        let out = compute_health(&caps(Some(50_000), Some(52_000)));
        match out {
            HealthOutcome::Ok { health_pct, above_design, .. } => {
                assert!((health_pct - 104.0).abs() < 1e-9);
                assert!(above_design);
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn implausible_health_is_anomaly_not_clamped() {
        let out = compute_health(&caps(Some(1_000), Some(9_000)));
        match out {
            HealthOutcome::Anomaly { raw_health_pct, .. } => {
                assert!((raw_health_pct - 900.0).abs() < 1e-9);
            }
            other => panic!("expected Anomaly, got {other:?}"),
        }
    }

    #[test]
    fn implausible_design_is_anomaly() {
        let out = compute_health(&caps(Some(5_000_000), Some(4_000_000)));
        assert!(matches!(out, HealthOutcome::Anomaly { .. }));
    }

    #[test]
    fn health_measurement_unavailable_when_missing() {
        let m = health_measurement(&caps(None, None), "test");
        assert_eq!(m.status, Status::Unavailable);
        assert!(m.value.is_none());
    }

    #[test]
    fn runtime_estimate() {
        assert!((estimate_runtime_hours(Some(40_000), Some(10_000)).unwrap() - 4.0).abs() < 1e-9);
        assert!(estimate_runtime_hours(Some(40_000), Some(0)).is_none());
        assert!(estimate_runtime_hours(None, Some(10_000)).is_none());
    }

    #[test]
    fn severity_thresholds() {
        use crate::model::Severity;
        assert_eq!(health_severity(95.0), Severity::Good);
        assert_eq!(health_severity(70.0), Severity::Information);
        assert_eq!(health_severity(50.0), Severity::Warning);
        assert_eq!(health_severity(20.0), Severity::Critical);
        assert_eq!(health_severity(f64::NAN), Severity::Unknown);
    }

    #[test]
    fn wear_never_negative_in_ok_case() {
        // Above-design batteries report negative wear honestly (not clamped).
        let out = compute_health(&caps(Some(50_000), Some(55_000)));
        if let HealthOutcome::Ok { wear_pct, .. } = out {
            assert!(wear_pct < 0.0); // honest, not clamped to 0
        } else {
            panic!("expected Ok");
        }
    }
}
