async function handleRequest(req, res) {
    let url = req.query.url;
    let response = await fetch(url);
    res.send(await response.text());
}
