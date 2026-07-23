async function fetchUserData(req: Request, res: Response) {
    const url = req.query.url;
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const target = req.body.target;
    const result = await fetch(target, {
        method: req.body.method,
        headers: req.body.headers,
    });
    const body = await result.text();
    res.send(body);
}

async function loadWebhook(req: Request, res: Response) {
    const webhookUrl = req.params.url;
    const resp = await fetch(webhookUrl);
    res.json({ status: resp.status });
}
