// SAFE: Uses reqwest::Client (connection pool enabled by default) which reuses connections via HTTP keep-alive.

fn fetch_url(host: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}{}", host, path);
    let response = client.get(&url).send()?.text()?;
    Ok(response)
}

fn fetch_two_urls(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let base = format!("http://{}", host);
    let r1 = client.get(format!("{}/api/data1", base)).send()?.text()?;
    let r2 = client.get(format!("{}/api/data2", base)).send()?.text()?;
    println!("{}{}", r1, r2);
    Ok(())
}
