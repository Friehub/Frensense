#![no_main]
use libfuzzer_sys::fuzz_target;
use gensense_auditor::{Engine, AstAuditor};
use std::path::Path;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let auditor = AstAuditor::default_auditor();
        let symbols = gensense_auditor::SymbolRegistry::new();
        // Use a dummy path that looks like a supported extension
        let path = Path::new("fuzz.rs");

        // We only care if this panics
        let _ = auditor.audit(path, s, &symbols, &Default::default(), &Default::default(), gensense_auditor::AuditorEnvironment::Development);
    }
});
