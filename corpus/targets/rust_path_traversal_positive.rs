fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
