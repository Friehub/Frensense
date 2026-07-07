// SPDX-License-Identifier: MIT

pub mod ast_diff;
pub mod auditor;
pub mod clustering;
pub mod composition;
pub mod confidence_calibration;
pub mod findings;
pub mod learn;
pub mod per_category_calibration;
pub mod project;
pub mod source;
pub mod suppression;

pub use auditor::FrensenseAuditor;
pub use project::Engine;
pub use suppression::{SuppressConfig, Suppression, is_suppressed};

// Re-export from engine crate for backward compatibility
pub use frensense_engine::fingerprint::FunctionFingerprint;
pub use frensense_engine::profile::ProjectProfile;
