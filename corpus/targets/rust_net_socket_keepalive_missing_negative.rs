use tokio::net::TcpStream;
use socket2::{Socket, Domain, Type, Protocol};

pub async fn connect(addr: &str) -> std::io::Result<TcpStream> {
    let sock = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    sock.set_keepalive(true)?;
    sock.set_tcp_keepalive(std::time::Duration::from_secs(60))?;
    sock.connect(&addr.parse().unwrap())?;
    TcpStream::from_std(sock.into())
}
