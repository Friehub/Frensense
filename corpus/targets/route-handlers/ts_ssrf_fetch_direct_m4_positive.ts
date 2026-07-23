// [frensense]
// observation: User-controlled URL is passed through a helper function that returns the URL without validation before fetch().
// impact: An attacker can make the server send requests to internal services by controlling the URL through an unsafe helper.
// improvement: Validate the URL inside the helper function against an allowlist.

function getUrl(req: Request): string {
    return req.query.url;
}

function getTarget(req: Request): string {
    return req.body.target;
}

async function fetchUserData(req: Request, res: Response) {
    const url = getUrl(req);
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const target = getTarget(req);
    const result = await fetch(target);
    const body = await result.text();
    res.send(body);
}
