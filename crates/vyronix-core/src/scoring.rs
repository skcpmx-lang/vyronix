//! Transparent, weighted device-condition scoring.
//!
//! Design rules (from the product requirements):
//!  * No invented universal health score. Every component score is derived
//!    from real, available measurements via documented rules.
//!  * Unsupported / unavailable components are EXCLUDED from the denominator,
//!    never awarded full points.
//!  * Coverage (how much of the weighted total was actually measurable) and
//!    confidence are reported alongside the score.
//!  * A low-coverage score is explicitly flagged as low-confidence.

use serde::{Deserialize, Serialize};

use crate::model::Severity;

/// One scored component (battery, storage, memory, thermals, ...).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentScore {
    pub id: String,
    pub label: String,
    /// Weight in the overall score (relative, not necessarily summing to 1).
    pub weight: f64,
    /// 0..=100, or None when the component could not be assessed.
    pub score: Option<f64>,
    pub severity: Severity,
    /// Whether this component contributed to the denominator.
    pub included: bool,
    pub rationale: String,
}

/// The aggregate result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverallScore {
    /// Weighted 0..=100 score over INCLUDED components only; None if nothing
    /// could be assessed.
    pub score: Option<f64>,
    /// Fraction of total weight that was measurable, 0..=1.
    pub coverage: f64,
    /// Number of components included vs total.
    pub included_count: usize,
    pub total_count: usize,
    /// Honest confidence label derived from coverage.
    pub confidence: ScoreConfidence,
    pub components: Vec<ComponentScore>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreConfidence {
    High,
    Medium,
    Low,
    Insufficient,
}

impl ScoreConfidence {
    pub fn label(self) -> &'static str {
        match self {
            ScoreConfidence::High => "High",
            ScoreConfidence::Medium => "Medium",
            ScoreConfidence::Low => "Low",
            ScoreConfidence::Insufficient => "Insufficient data",
        }
    }
}

/// Map a severity to a component score using documented, fixed anchors.
/// Unknown maps to None (excluded), never to a good score.
pub fn severity_to_score(sev: Severity) -> Option<f64> {
    match sev {
        Severity::Good => Some(100.0),
        Severity::Information => Some(80.0),
        Severity::Warning => Some(50.0),
        Severity::Critical => Some(15.0),
        Severity::Unknown => None,
    }
}

/// Compute the overall score from component scores. Components with
/// `score == None` are excluded from the denominator.
pub fn compute_overall(components: Vec<ComponentScore>) -> OverallScore {
    let total_weight: f64 = components.iter().map(|c| c.weight).sum();
    let mut included_weight = 0.0;
    let mut weighted_sum = 0.0;
    let mut included_count = 0;

    for c in &components {
        if let Some(s) = c.score {
            if c.included && c.weight > 0.0 {
                included_weight += c.weight;
                weighted_sum += s * c.weight;
                included_count += 1;
            }
        }
    }

    let coverage = if total_weight > 0.0 {
        included_weight / total_weight
    } else {
        0.0
    };
    let score = if included_weight > 0.0 {
        Some(weighted_sum / included_weight)
    } else {
        None
    };

    let confidence = if included_count == 0 {
        ScoreConfidence::Insufficient
    } else if coverage >= 0.75 {
        ScoreConfidence::High
    } else if coverage >= 0.5 {
        ScoreConfidence::Medium
    } else {
        ScoreConfidence::Low
    };

    OverallScore {
        score,
        coverage,
        included_count,
        total_count: components.len(),
        confidence,
        components,
    }
}

/// Helper to build a component score from a severity.
pub fn component(
    id: &str,
    label: &str,
    weight: f64,
    sev: Severity,
    rationale: &str,
) -> ComponentScore {
    let score = severity_to_score(sev);
    ComponentScore {
        id: id.into(),
        label: label.into(),
        weight,
        score,
        severity: sev,
        included: score.is_some(),
        rationale: rationale.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_excluded_from_denominator() {
        let comps = vec![
            component("battery", "Battery", 2.0, Severity::Good, "90% health"),
            component(
                "storage",
                "Storage",
                3.0,
                Severity::Unknown,
                "SMART unavailable",
            ),
        ];
        let overall = compute_overall(comps);
        // Only battery counts: score must be 100, not (100+100)/2 diluted by storage.
        assert_eq!(overall.score, Some(100.0));
        assert_eq!(overall.included_count, 1);
        assert_eq!(overall.total_count, 2);
        assert!((overall.coverage - 0.4).abs() < 1e-9); // 2 / (2+3)
        assert_eq!(overall.confidence, ScoreConfidence::Low);
    }

    #[test]
    fn unknown_never_awarded_points() {
        assert_eq!(severity_to_score(Severity::Unknown), None);
    }

    #[test]
    fn all_unknown_yields_no_score() {
        let comps = vec![
            component("a", "A", 1.0, Severity::Unknown, ""),
            component("b", "B", 1.0, Severity::Unknown, ""),
        ];
        let overall = compute_overall(comps);
        assert_eq!(overall.score, None);
        assert_eq!(overall.coverage, 0.0);
        assert_eq!(overall.confidence, ScoreConfidence::Insufficient);
    }

    #[test]
    fn weighted_average_correct() {
        let comps = vec![
            component("a", "A", 1.0, Severity::Good, ""),    // 100
            component("b", "B", 3.0, Severity::Warning, ""), // 50
        ];
        let overall = compute_overall(comps);
        // (100*1 + 50*3) / 4 = 62.5
        assert_eq!(overall.score, Some(62.5));
        assert_eq!(overall.coverage, 1.0);
        assert_eq!(overall.confidence, ScoreConfidence::High);
    }

    #[test]
    fn empty_components_insufficient() {
        let overall = compute_overall(vec![]);
        assert_eq!(overall.score, None);
        assert_eq!(overall.confidence, ScoreConfidence::Insufficient);
    }

    #[test]
    fn coverage_confidence_tiers() {
        let mk = |cov_good: usize, total: usize| {
            let mut comps = Vec::new();
            for i in 0..total {
                let sev = if i < cov_good {
                    Severity::Good
                } else {
                    Severity::Unknown
                };
                comps.push(component(&format!("c{i}"), "C", 1.0, sev, ""));
            }
            compute_overall(comps)
        };
        assert_eq!(mk(4, 4).confidence, ScoreConfidence::High);
        assert_eq!(mk(3, 4).confidence, ScoreConfidence::High);
        assert_eq!(mk(2, 4).confidence, ScoreConfidence::Medium);
        assert_eq!(mk(1, 4).confidence, ScoreConfidence::Low);
    }

    #[test]
    fn serialization_roundtrip() {
        let comps = vec![component("battery", "Battery", 2.0, Severity::Good, "ok")];
        let overall = compute_overall(comps);
        let json = serde_json::to_string(&overall).unwrap();
        let back: OverallScore = serde_json::from_str(&json).unwrap();
        assert_eq!(overall, back);
    }
}
