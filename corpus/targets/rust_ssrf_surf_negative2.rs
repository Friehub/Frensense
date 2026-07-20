// SAFE: Uses IP range checking and rejects private addresses
use surf;
use std::net::{IpAddr, ToSocketAddrs};

async fn external_fetch(url: &str) -> Result<String, surf::Error> {
    let parsed = url::Url::parse(url).map_err(|_| surf::Error::new("invalid URL"))?;
    let host = parsed.host_str().ok_or_else(|| surf::Error::new("no host"))?;
    let ips: Vec<IpAddr> = host.to_socket_addrs()
        .map_err(|_| surf::Error::new("dns failed"))?
        .map(|a| a.ip())
        .collect();
    for ip in &ips {
        if ip.is_loopback() || ip.is_private() || ip.is_unspecified() {
            return Err(surf::Error::new("internal address blocked"));
        }
    }
    let mut resp = surf::get(url).await?;
    Ok(resp.body_string().await?)
}
