// SPDX-License-Identifier: MIT

use crate::GenSenseError;

pub fn safe_source_slice(source: &str, start_byte: usize, end_byte: usize) -> Result<&str, GenSenseError> {
    if start_byte > end_byte || end_byte > source.len() {
        return Err(GenSenseError::Engine(format!(
            "Invalid byte range [{start_byte}..{end_byte}] for source of length {}",
            source.len()
        )));
    }
    source.get(start_byte..end_byte).ok_or_else(|| {
        GenSenseError::Engine(format!(
            "Failed to slice source at [{start_byte}..{end_byte}] (len={})",
            source.len()
        ))
    })
}
