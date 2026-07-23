// [frensense]
// observation: User-controlled URL is passed to fetch() without validation inside a conditional block on the tainted branch.
// impact: An attacker can make the server send requests to internal services.
// improvement: Validate URL against an allowlist

async function fetchUserData(req: Request, res: Response) {
    if (req.query.url) {
        const response = await fetch(req.query.url);
        const data = await response.json(); res.json(data);
    } else { res.status(400).send("Missing URL"); }
}

async function proxyRequest(req: Request, res: Response) {
    if (req.body.target && req.body.target.length > 0) {
        const result = await fetch(req.body.target);
        const body = await result.text(); res.send(body);
    }
}
