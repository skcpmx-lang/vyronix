//! Honest measurement model.
//!
//! Every hardware value is wrapped in a [`Measurement`] that records its
//! availability [`Status`], [`Confidence`], data `source` and an optional
//! human-readable `note`. Unknown is never reported as Good; estimated values
//! are never presented as directly measured.

use serde::{Deserialize, Serialize};

/// Availability / provenance classification for a single value or test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Retrieved successfully from a supported hardware interface.
    Available,
    /// Successfully validated through an actual test.
    Verified,
    /// Calculated through a documented estimation method.
    Estimated,
    /// Hardware or OS does not expose the information.
    Unsupported,
    /// Administrator privileges or additional access are needed.
    PermissionRequired,
    /// Information could not be retrieved.
    Unavailable,
    /// An actual test was attempted and failed.
    Failed,
    /// No test has been performed.
    NotTested,
    /// Physical inspection is necessary.
    ManualVerificationRequired,
}

impl Status {
    /// Whether a concrete value is present and trustworthy enough to display
    /// as a number. Estimated values are present but must be labelled.
    pub fn has_value(self) -> bool {
        matches!(
            self,
            Status::Available | Status::Verified | Status::Estimated
        )
    }

    /// Whether this status represents a problem that should be surfaced.
    pub fn is_problem(self) -> bool {
        matches!(self, Status::Failed | Status::Unavailable)
    }

    pub fn label(self) -> &'static str {
        match self {
            Status::Available => "Available",
            Status::Verified => "Verified",
            Status::Estimated => "Estimated",
            Status::Unsupported => "Unsupported",
            Status::PermissionRequired => "Permission Required",
            Status::Unavailable => "Unavailable",
            Status::Failed => "Failed",
            Status::NotTested => "Not Tested",
            Status::ManualVerificationRequired => "Manual Verification Required",
        }
    }
}

/// Confidence that a reported value reflects physical reality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    High,
    Medium,
    Low,
    None,
}

/// Diagnostic severity. `Unknown` is deliberately distinct from `Good`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Good,
    Information,
    Warning,
    Critical,
    Unknown,
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Severity::Good => "Good",
            Severity::Information => "Information",
            Severity::Warning => "Warning",
            Severity::Critical => "Critical",
            Severity::Unknown => "Unknown",
        }
    }

    /// Numeric weight for ordering; higher is more severe.
    pub fn rank(self) -> u8 {
        match self {
            Severity::Good => 0,
            Severity::Information => 1,
            Severity::Unknown => 2,
            Severity::Warning => 3,
            Severity::Critical => 4,
        }
    }
}

/// A single honest measurement.
///
/// `value` is `None` whenever the status does not carry a concrete value.
/// `source` names the interface the value came from (e.g. "Windows
/// IOCTL_BATTERY_QUERY_INFORMATION", "WMI Win32_Processor", "sysinfo").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement<T> {
    pub value: Option<T>,
    pub status: Status,
    pub confidence: Confidence,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Unix milliseconds when the value was collected (0 if not recorded).
    pub timestamp_ms: u64,
}

impl<T> Measurement<T> {
    pub fn new(
        value: T,
        status: Status,
        confidence: Confidence,
        source: impl Into<String>,
    ) -> Self {
        Self {
            value: Some(value),
            status,
            confidence,
            source: source.into(),
            note: None,
            timestamp_ms: 0,
        }
    }

    pub fn available(value: T, source: impl Into<String>) -> Self {
        Self::new(value, Status::Available, Confidence::High, source)
    }

    pub fn estimated(value: T, source: impl Into<String>, note: impl Into<String>) -> Self {
        let mut m = Self::new(value, Status::Estimated, Confidence::Medium, source);
        m.note = Some(note.into());
        m
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_timestamp(mut self, ts: u64) -> Self {
        self.timestamp_ms = ts;
        self
    }

    /// Construct a value-less measurement for a non-available status.
    pub fn empty(status: Status, source: impl Into<String>, note: impl Into<String>) -> Self {
        Self {
            value: None,
            status,
            confidence: if status == Status::Unsupported || status == Status::NotTested {
                Confidence::None
            } else {
                Confidence::Low
            },
            source: source.into(),
            note: Some(note.into()),
            timestamp_ms: 0,
        }
    }

    pub fn unsupported(source: impl Into<String>, note: impl Into<String>) -> Self {
        Self::empty(Status::Unsupported, source, note)
    }

    pub fn unavailable(source: impl Into<String>, note: impl Into<String>) -> Self {
        Self::empty(Status::Unavailable, source, note)
    }

    pub fn permission_required(source: impl Into<String>, note: impl Into<String>) -> Self {
        Self::empty(Status::PermissionRequired, source, note)
    }

    pub fn not_tested(source: impl Into<String>) -> Self {
        Self::empty(Status::NotTested, source, "No test has been performed yet.")
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Measurement<U> {
        Measurement {
            value: self.value.map(f),
            status: self.status,
            confidence: self.confidence,
            source: self.source,
            note: self.note,
            timestamp_ms: self.timestamp_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_is_not_good() {
        assert_ne!(Severity::Unknown, Severity::Good);
        assert!(Severity::Unknown.rank() > Severity::Good.rank());
    }

    #[test]
    fn status_value_presence() {
        assert!(Status::Available.has_value());
        assert!(Status::Verified.has_value());
        assert!(Status::Estimated.has_value());
        assert!(!Status::Unsupported.has_value());
        assert!(!Status::Unavailable.has_value());
        assert!(!Status::NotTested.has_value());
    }

    #[test]
    fn estimated_is_labelled() {
        let m = Measurement::estimated(42.0, "test", "derived from X");
        assert_eq!(m.status, Status::Estimated);
        assert_eq!(m.note.as_deref(), Some("derived from X"));
        assert_ne!(m.status, Status::Available);
    }

    #[test]
    fn empty_has_no_value() {
        let m: Measurement<f64> = Measurement::unsupported("WMI", "not exposed");
        assert!(m.value.is_none());
        assert_eq!(m.confidence, Confidence::None);
    }

    #[test]
    fn serialization_roundtrip() {
        let m = Measurement::available(97.5_f64, "IOCTL_BATTERY_QUERY_INFORMATION");
        let json = serde_json::to_string(&m).unwrap();
        let back: Measurement<f64> = serde_json::from_str(&json).unwrap();
        assert_eq!(m, back);
        assert!(json.contains("\"status\":\"available\""));
    }

    #[test]
    fn severity_ordering() {
        assert!(Severity::Critical.rank() > Severity::Warning.rank());
        assert!(Severity::Warning.rank() > Severity::Information.rank());
    }
}
