// [frensense]
// observation: User-controlled URL is passed to fetch() without validation across an async/await boundary.
// impact: An attacker can make the server send requests to internal services.
// improvement: Validate the URL against an allowlist of permitted hosts

async function getUrl(req: any): Promise<string> { return req.query.url; }
async function getTarget(req: any): Promise<string> { return req.body.target; }

async function fetchUserData(req: Request, res: Response) {
    const url = await getUrl(req);
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const target = await getTarget(req);
    const result = await fetch(target);
    const body = await result.text();
    res.send(body);
}
