use std::net::TcpStream;

fn transfer_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let conn = TcpStream::connect(url)?;
    let result = (|| {
        let mut buf = [0u8; 4096];
        let n = conn.read(&mut buf)?;
        let data = String::from_utf8_lossy(&buf[..n]).to_string();

        if data.is_empty() {
            return Err("empty response".into());
        }

        Ok(data)
    })();

    drop(conn);
    result
}
