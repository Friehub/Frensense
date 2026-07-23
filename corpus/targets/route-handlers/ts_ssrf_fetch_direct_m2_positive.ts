// [frensense]
// observation: User-controlled URL flows through an intermediate variable into fetch() without validation, enabling SSRF.
// impact: An attacker can make the server send requests to internal services by controlling the URL parameter.
// improvement: Validate the URL against an allowlist of permitted hosts before fetching.

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
