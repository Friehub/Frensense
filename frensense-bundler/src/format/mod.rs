// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! The `.frc` bundle: format envelope + payload types.
//!
//! The bundler owns the format end to end. The engine never sees bundle
//! bytes, it only consumes the decoded [`LearnedFactEntry`] facts, which
//! live in `frensense_engine::analysis::taint::facts`.

mod frc;
mod types;

pub use frc::{read_bundle, write_bundle, BundleHeader, BUNDLE_MAGIC, BUNDLE_VERSION};
pub use types::{load_bundle, BundlePattern, BundlePayload, LoadedBundle};
