async function handleRequest(req, res) {
    let url = req.query.url;
    let safeUrl = sanitizeUrl(url);
    let response = await fetch(safeUrl);
    res.send(await response.text());
}
