// SAFE: TCP keepalive is enabled with a 60-second idle timeout.
use tokio::net::TcpStream;
use std::time::Duration;

pub async fn connect(addr: &str) -> std::io::Result<TcpStream> {
    let stream = TcpStream::connect(addr).await?;
    stream.set_keepalive(Some(Duration::from_secs(60)))?;
    Ok(stream)
}
