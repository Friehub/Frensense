fn read_file(path: &str) -> Result<String, io::Error> {
    let safe_path = Path::new("/uploads").join(Path::new(path).file_name().unwrap_or_default());
    fs::read_to_string(safe_path)
}
