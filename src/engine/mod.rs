// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! The consumer engine: a thin driver over `frensense_engine::scan`.
//!
//! There is no rule framework, no fingerprint matching, no confidence
//! scoring. The engine collects files, lowers them through the shared
//! harness, and turns flow verdicts into advisories, compiler-style.

pub mod files;
pub mod project;

pub use project::Engine;
