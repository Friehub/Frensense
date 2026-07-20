const ALLOWED_HOSTS = ["api.trusted.com", "data.internal.com"];

function isHostSafe(host: string): boolean {
    return ALLOWED_HOSTS.includes(host);
}

async function fetchUserData(req: Request, res: Response) {
    const host = req.query.url;
    if (!isHostSafe(host)) return res.status(403).json({ error: "Host not allowed" });
    const response = await fetch(`https://${host}`);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const target = req.body.target;
    const parsed = new URL(target);
    if (!isHostSafe(parsed.hostname)) return res.status(403).json({ error: "Host not allowed" });
    const result = await fetch(`${target}/api/data`);
    const body = await result.text();
    res.send(body);
}
