// SAFE: Object property URL validated against allowlist
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];

function isValidUrl(url: string): boolean {
    try {
        const parsed = new URL(url);
        return ALLOWED_HOSTS.includes(parsed.hostname);
    } catch { return false; }
}

async function fetchUserData(req: Request, res: Response) {
    const cfg = { url: req.query.url };
    if (!isValidUrl(cfg.url)) return res.status(403).send("Host not allowed");
    const response = await fetch(cfg.url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const opts = { target: req.body.target };
    if (!isValidUrl(opts.target)) return res.status(403).send("Host not allowed");
    const result = await fetch(opts.target);
    const body = await result.text();
    res.send(body);
}
