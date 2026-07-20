// [frensense]
// observation: User-controlled URL is accessed through an array element and passed to fetch() without validation.
// impact: An attacker can make the server send requests to internal services by controlling an array-indexed URL.
// improvement: Validate the array element URL against an allowlist before fetching.

async function fetchUserData(req: Request, res: Response) {
    const urls = [req.query.url];
    const response = await fetch(urls[0]);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const targets = [req.body.target];
    const result = await fetch(targets[0]);
    const body = await result.text();
    res.send(body);
}
