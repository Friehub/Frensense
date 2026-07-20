const ALLOWED_HOSTS = ["api.trusted.com", "data.internal.com"];

function isUrlSafe(url: string): boolean {
    try {
        const parsed = new URL(url);
        return ALLOWED_HOSTS.includes(parsed.hostname);
    } catch {
        return false;
    }
}

async function fetchUserData(req: Request, res: Response) {
    const url = req.query.url;
    if (!isUrlSafe(url)) return res.status(403).json({ error: "URL not allowed" });
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const target = req.body.target;
    if (!isUrlSafe(target)) return res.status(403).json({ error: "URL not allowed" });
    const result = await fetch(target, {
        method: req.body.method,
        headers: req.body.headers,
    });
    const body = await result.text();
    res.send(body);
}
