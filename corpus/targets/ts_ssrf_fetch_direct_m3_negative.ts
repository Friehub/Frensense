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
    const a = req.query.url;
    const b = a;
    if (!isUrlSafe(b)) return res.status(403).json({ error: "URL not allowed" });
    const response = await fetch(b);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const raw = req.body.target;
    const target = raw;
    if (!isUrlSafe(target)) return res.status(403).json({ error: "URL not allowed" });
    const result = await fetch(target);
    const body = await result.text();
    res.send(body);
}
