// SPDX-License-Identifier: MIT

pub mod ast_diff;
pub mod auditor;
pub mod clustering;
pub mod composition;
pub mod confidence_calibration;
pub mod findings;
pub mod learn;
pub mod per_category_calibration;
#[cfg(feature = "fingerprinting")]
pub mod fingerprint;
#[cfg(feature = "fingerprinting")]
pub mod profile;
pub mod project;
pub mod source;
pub mod suppression;

pub use auditor::FrensenseAuditor;
#[cfg(feature = "fingerprinting")]
pub use fingerprint::FunctionFingerprint;
#[cfg(feature = "fingerprinting")]
pub use profile::ProjectProfile;
pub use project::Engine;
pub use suppression::{SuppressConfig, Suppression, is_suppressed};
