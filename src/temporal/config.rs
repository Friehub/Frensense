// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TemporalConfig {
    pub sequence: Vec<String>,
    pub behavior: String,
}
