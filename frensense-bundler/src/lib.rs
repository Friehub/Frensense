// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

#![allow(clippy::all)]
#![allow(dead_code, unreachable_patterns, unreachable_code)]

//! Frensense bundler: builds `.frc` bundles from positive/negative corpus
//! pairs. The learning pipeline is: group families → scan both variants
//! through the engine's compiler → delta → replay gate → publish facts.

pub mod builder;
pub mod fact_extract;
pub mod format;
