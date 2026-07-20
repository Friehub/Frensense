// [frensense]
// observation: User-controlled URL is passed to fetch() without validation, enabling SSRF through an object property.
// impact: An attacker can make the server send requests to internal services by controlling the URL.
// improvement: Validate the URL against an allowlist of permitted hosts before fetching

async function fetchUserData(req: Request, res: Response) {
    const cfg = { url: req.query.url };
    const response = await fetch(cfg.url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const opts = { target: req.body.target };
    const result = await fetch(opts.target);
    const body = await result.text();
    res.send(body);
}
