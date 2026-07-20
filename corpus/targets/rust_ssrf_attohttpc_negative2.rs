// SAFE: Resolves hostname and rejects private IP ranges
use attohttpc;
use std::net::ToSocketAddrs;

fn fetch_url(url: &str) -> Result<String, attohttpc::Error> {
    let parsed = url::Url::parse(url).map_err(|_| attohttpc::Error::new("bad url"))?;
    let host = parsed.host_str().ok_or_else(|| attohttpc::Error::new("no host"))?;
    let addrs: Vec<_> = host.to_socket_addrs()
        .map_err(|_| attohttpc::Error::new("dns failed"))?
        .collect();
    for addr in &addrs {
        if addr.ip().is_loopback() || addr.ip().is_private() {
            return Err(attohttpc::Error::new("private address blocked"));
        }
    }
    let resp = attohttpc::get(url).send()?;
    Ok(resp.text()?)
}
