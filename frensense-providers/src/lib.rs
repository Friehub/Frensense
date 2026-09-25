// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

#![allow(clippy::all)]

#[cfg(feature = "oxc")]
pub mod oxc_provider;

#[cfg(feature = "rust-hir")]
pub mod rust_hir_provider;