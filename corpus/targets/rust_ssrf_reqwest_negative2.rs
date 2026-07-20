// SAFE: Uses a reqwest::Client with a custom DNS resolver that blocks internal IPs; no raw URL validation needed at call site
use reqwest;
use std::sync::Arc;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use reqwest::dns::{Addrs, Resolve, Resolving};
use std::net::SocketAddr;

struct SafeDnsResolver;

impl Resolve for SafeDnsResolver {
    fn resolve(&self, _name: reqwest::dns::Name) -> Resolving {
        Box::pin(async move {
            let resolver = hickory_resolver::TokioAsyncResolver::new(
                ResolverConfig::default(),
                ResolverOpts::default(),
            );
            let response = resolver.lookup_ip(_name.as_str().to_string()).await?;
            let addrs: Vec<SocketAddr> = response
                .iter()
                .filter(|ip| {
                    !(ip.is_loopback()
                        || ip.is_private()
                        || ip.is_unspecified()
                        || ip.is_link_local())
                })
                .map(|ip| SocketAddr::new(ip, 0))
                .collect();
            if addrs.is_empty() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "all resolved IPs are blocked",
                )) as Box<dyn std::error::Error + Send + Sync>);
            }
            Ok(Box::new(addrs.into_iter()) as Box<dyn Iterator<Item = SocketAddr> + Send>)
        })
    }
}

async fn fetch_external(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .dns_resolver(Arc::new(SafeDnsResolver))
        .build()?;
    let resp = client.get(url).send().await?;
    Ok(resp.text().await?)
}
