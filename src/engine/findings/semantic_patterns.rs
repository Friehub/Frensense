// SPDX-License-Identifier: MIT

use crate::Advisory;
use crate::engine::project::FileSnapshot;

/// Run semantic pattern detectors on a file snapshot.
///
/// Semantic patterns are now corpus-driven. This module exists as a no-op
/// placeholder for the finding module registration system.
#[must_use]
pub fn find(_snap: &FileSnapshot) -> Vec<Advisory> {
    Vec::new()
}
