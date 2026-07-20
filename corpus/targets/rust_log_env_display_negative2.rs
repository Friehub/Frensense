// SAFE: Implements a separate method for internal logging that redacts the secret portion
use std::fmt;

struct ApiKey {
    value: String,
    label: String,
}

impl ApiKey {
    fn masked(&self) -> String {
        format!("{}...{}", &self.label, &self.value[self.value.len().saturating_sub(4)..])
    }
}
