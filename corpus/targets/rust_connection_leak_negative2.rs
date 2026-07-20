// SAFE: Uses a Drop-based wrapper that guarantees connection cleanup
use std::io::{Read, Write};
use std::net::TcpStream;

struct SafeConnection {
    stream: TcpStream,
}

impl SafeConnection {
    fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(url)?;
        Ok(SafeConnection { stream })
    }

    fn read_all(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        self.stream.read_to_end(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

fn transfer_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut conn = SafeConnection::connect(url)?;
    conn.read_all()
}
