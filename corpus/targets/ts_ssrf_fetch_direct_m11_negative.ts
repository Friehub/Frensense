// SAFE: Conditional branch with URL validation
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];
function isValidUrl(url: string): boolean {
    try { const parsed = new URL(url); return ALLOWED_HOSTS.includes(parsed.hostname); } catch { return false; }
}

async function fetchUserData(req: Request, res: Response) {
    if (req.query.url) {
        if (!isValidUrl(req.query.url)) return res.status(403).send("Host not allowed");
        const response = await fetch(req.query.url); const data = await response.json(); res.json(data);
    } else { res.status(400).send("Missing URL"); }
}

async function proxyRequest(req: Request, res: Response) {
    if (req.body.target && req.body.target.length > 0) {
        if (!isValidUrl(req.body.target)) return res.status(403).send("Host not allowed");
        const result = await fetch(req.body.target); const body = await result.text(); res.send(body);
    }
}
