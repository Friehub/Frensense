// [frensense]
// observation: User-controlled URL is passed to fetch() without validation with renamed variables.
// impact: An attacker can make the server send requests to internal services.
// improvement: Validate URL against allowlist

async function fetchUserData(req: Request, res: Response) {
    const userProvidedUrl = req.query.url;
    const response = await fetch(userProvidedUrl);
    const data = await response.json(); res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const requestTarget = req.body.target;
    const result = await fetch(requestTarget);
    const body = await result.text(); res.send(body);
}
