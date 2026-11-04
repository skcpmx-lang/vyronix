//! VYRONIX core: platform-independent diagnostic logic.
//!
//! This crate contains NO hardware access and NO fake data. It defines the
//! honest measurement model, validated calculations (battery, SMART), a
//! transparent scoring engine, deterministic benchmark kernels, diagnostic
//! rules and report serialization. All hardware collection lives in the
//! Tauri backend and feeds real values into these types.

pub mod model;
pub mod battery;
pub mod smart;
pub mod scoring;
pub mod bench;
pub mod validation;
pub mod recommend;
pub mod diagnostics;
pub mod report;

pub use model::{Confidence, Measurement, Severity, Status};

/// Library version, kept in sync with the workspace VERSION file.
pub const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
