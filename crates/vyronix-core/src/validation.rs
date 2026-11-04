//! Input validation and filesystem-path sanitization.
//!
//! The frontend is untrusted input. These helpers enforce bounds and reject
//! path traversal / injection before any native operation runs.

use std::path::{Component, Path, PathBuf};

/// Errors produced by validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    #[error("value {value} is out of range [{min}, {max}]")]
    OutOfRange { value: f64, min: f64, max: f64 },
    #[error("path is empty")]
    EmptyPath,
    #[error("path contains traversal or invalid components")]
    InvalidPath,
    #[error("path is not absolute")]
    NotAbsolute,
    #[error("string exceeds maximum length {max}")]
    TooLong { max: usize },
    #[error("string contains control characters")]
    ControlChars,
}

/// Validate that a number lies within an inclusive range and is finite.
pub fn range_f64(value: f64, min: f64, max: f64) -> Result<f64, ValidationError> {
    if !value.is_finite() || value < min || value > max {
        return Err(ValidationError::OutOfRange { value, min, max });
    }
    Ok(value)
}

/// Validate an integer range.
pub fn range_u64(value: u64, min: u64, max: u64) -> Result<u64, ValidationError> {
    if value < min || value > max {
        return Err(ValidationError::OutOfRange {
            value: value as f64,
            min: min as f64,
            max: max as f64,
        });
    }
    Ok(value)
}

/// Reject control characters (except none) and over-long strings. Used for
/// notes, labels and any free text that reaches logs or reports.
pub fn clean_text(s: &str, max_len: usize) -> Result<String, ValidationError> {
    if s.chars().count() > max_len {
        return Err(ValidationError::TooLong { max: max_len });
    }
    if s.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
        return Err(ValidationError::ControlChars);
    }
    Ok(s.to_string())
}

/// Sanitize an export destination path.
///
/// Rules:
///  * must be absolute,
///  * must not contain `..` (parent-dir traversal),
///  * must not contain NUL or control characters,
///  * must end with an allowed report extension.
///
/// Returns a normalized PathBuf on success. This does NOT touch the filesystem.
pub fn sanitize_export_path(
    path: &str,
    allowed_extensions: &[&str],
) -> Result<PathBuf, ValidationError> {
    if path.trim().is_empty() {
        return Err(ValidationError::EmptyPath);
    }
    if path.chars().any(|c| c.is_control()) {
        return Err(ValidationError::ControlChars);
    }
    let p = Path::new(path);
    if !p.is_absolute() {
        return Err(ValidationError::NotAbsolute);
    }
    for comp in p.components() {
        match comp {
            Component::ParentDir => return Err(ValidationError::InvalidPath),
            Component::Normal(_) | Component::RootDir | Component::Prefix(_) => {}
            Component::CurDir => return Err(ValidationError::InvalidPath),
        }
    }
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());
    match ext {
        Some(e)
            if allowed_extensions
                .iter()
                .any(|a| a.eq_ignore_ascii_case(&e)) => {}
        _ => return Err(ValidationError::InvalidPath),
    }
    Ok(p.to_path_buf())
}

/// Validate a monitoring poll interval in milliseconds.
pub fn poll_interval_ms(value: u64) -> Result<u64, ValidationError> {
    range_u64(value, 250, 60_000)
}

/// Validate a benchmark duration in seconds.
pub fn bench_duration_secs(value: u64) -> Result<u64, ValidationError> {
    range_u64(value, 1, 600)
}

/// Validate a storage benchmark file size in MiB (bounded to protect disks).
pub fn storage_test_size_mib(value: u64) -> Result<u64, ValidationError> {
    range_u64(value, 16, 4096)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_accepts_and_rejects() {
        assert!(range_f64(5.0, 0.0, 10.0).is_ok());
        assert!(range_f64(11.0, 0.0, 10.0).is_err());
        assert!(range_f64(f64::NAN, 0.0, 10.0).is_err());
        assert!(range_f64(f64::INFINITY, 0.0, 10.0).is_err());
    }

    #[test]
    fn clean_text_rejects_control_and_length() {
        assert!(clean_text("hello", 10).is_ok());
        assert!(clean_text("hello\nworld", 20).is_ok());
        assert!(clean_text("bad\u{0}text", 20).is_err());
        assert!(clean_text("toolong", 3).is_err());
    }

    #[test]
    fn export_path_rejects_traversal() {
        // Use a platform-appropriate absolute path.
        #[cfg(windows)]
        let base = "C:\\Users\\test";
        #[cfg(not(windows))]
        let base = "/home/test";

        let ok = format!("{base}/report.json");
        assert!(sanitize_export_path(&ok, &["json", "html", "csv", "pdf"]).is_ok());

        let traversal = format!("{base}/../evil.json");
        assert!(sanitize_export_path(&traversal, &["json"]).is_err());

        let wrong_ext = format!("{base}/report.exe");
        assert!(sanitize_export_path(&wrong_ext, &["json"]).is_err());

        assert!(sanitize_export_path("", &["json"]).is_err());
        assert!(sanitize_export_path("relative/report.json", &["json"]).is_err());
    }

    #[test]
    fn export_path_rejects_nul() {
        #[cfg(windows)]
        let p = "C:\\Users\\test\\re\u{0}port.json";
        #[cfg(not(windows))]
        let p = "/home/test/re\u{0}port.json";
        assert!(sanitize_export_path(p, &["json"]).is_err());
    }

    #[test]
    fn poll_interval_bounds() {
        assert!(poll_interval_ms(1000).is_ok());
        assert!(poll_interval_ms(100).is_err());
        assert!(poll_interval_ms(120_000).is_err());
    }

    #[test]
    fn bench_duration_bounds() {
        assert!(bench_duration_secs(30).is_ok());
        assert!(bench_duration_secs(0).is_err());
        assert!(bench_duration_secs(3600).is_err());
    }

    #[test]
    fn storage_size_bounds() {
        assert!(storage_test_size_mib(256).is_ok());
        assert!(storage_test_size_mib(1).is_err());
        assert!(storage_test_size_mib(100_000).is_err());
    }
}
