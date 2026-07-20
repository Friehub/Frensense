// [frensense]
// observation: User-controlled URL traverses multiple variable assignments before reaching fetch() without validation.
// impact: An attacker can make the server send requests to internal services through multi-hop assignment.
// improvement: Validate the URL against an allowlist regardless of how many assignment hops occur.

async function fetchUserData(req: Request, res: Response) {
    const a = req.query.url;
    const b = a;
    const response = await fetch(b);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const raw = req.body.target;
    const target = raw;
    const result = await fetch(target);
    const body = await result.text();
    res.send(body);
}
