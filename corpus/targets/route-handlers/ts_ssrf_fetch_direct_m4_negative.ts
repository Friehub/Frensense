const ALLOWED_HOSTS = ["api.trusted.com", "data.internal.com"];

function getValidatedUrl(req: Request): string {
    const url = req.query.url;
    const parsed = new URL(url);
    if (!ALLOWED_HOSTS.includes(parsed.hostname)) {
        throw new Error("Host not allowed");
    }
    return url;
}

function getValidatedTarget(req: Request): string {
    const target = req.body.target;
    const parsed = new URL(target);
    if (!ALLOWED_HOSTS.includes(parsed.hostname)) {
        throw new Error("Host not allowed");
    }
    return target;
}

async function fetchUserData(req: Request, res: Response) {
    try {
        const url = getValidatedUrl(req);
        const response = await fetch(url);
        const data = await response.json();
        res.json(data);
    } catch {
        res.status(403).json({ error: "URL not allowed" });
    }
}

async function proxyRequest(req: Request, res: Response) {
    try {
        const target = getValidatedTarget(req);
        const result = await fetch(target);
        const body = await result.text();
        res.send(body);
    } catch {
        res.status(403).json({ error: "URL not allowed" });
    }
}
