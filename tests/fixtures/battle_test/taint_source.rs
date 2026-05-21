fn get_sensitive_data() -> String {
    "SECRET_KEY".to_string()
}

fn trigger_leak() {
    let key = get_sensitive_data();
    // This calls a function in another file
    crate::leak_target::process_data(key);
}
