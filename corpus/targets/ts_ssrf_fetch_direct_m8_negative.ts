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
    const urls = [req.query.url];
    if (!isUrlSafe(urls[0])) return res.status(403).json({ error: "URL not allowed" });
    const response = await fetch(urls[0]);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const targets = [req.body.target];
    if (!isUrlSafe(targets[0])) return res.status(403).json({ error: "URL not allowed" });
    const result = await fetch(targets[0]);
    const body = await result.text();
    res.send(body);
}
