// SPDX-License-Identifier: MIT

pub mod auditor;
pub mod findings;
#[cfg(feature = "fingerprinting")]
pub mod fingerprint;
#[cfg(feature = "fingerprinting")]
pub mod profile;
pub mod project;
pub mod source;
pub mod suppression;
pub mod taint_rules;

pub use auditor::FrensenseAuditor;
#[cfg(feature = "fingerprinting")]
pub use fingerprint::FunctionFingerprint;
#[cfg(feature = "fingerprinting")]
pub use profile::ProjectProfile;
pub use project::Engine;
pub use suppression::{SuppressConfig, Suppression, is_suppressed};
