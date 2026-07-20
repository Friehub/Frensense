// [frensense]
// observation: A TCP `TcpStream` is created without setting `SO_KEEPALIVE`. Idle connections remain open indefinitely, consuming server resources.
// impact: Connection leak — zombie connections accumulate, eventually exhausting the file descriptor limit and causing denial of service.
// improvement: Set `SO_KEEPALIVE` with `socket2::Socket::set_keepalive` or `TcpStream::set_keepalive`.

use tokio::net::TcpStream;

pub async fn connect(addr: &str) -> std::io::Result<TcpStream> {
    let stream = TcpStream::connect(addr).await?;
    Ok(stream)
}
