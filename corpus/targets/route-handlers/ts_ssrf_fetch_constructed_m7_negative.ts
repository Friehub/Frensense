// SAFE: Implements safe alternative
// SAFE: URL validated against allowlist before fetch
const ALLOWED_HOSTS = ["api.trusted.com", "data.internal.com"];
function isUrlSafe(url: string): boolean {
    try { const parsed = new URL(url); return ALLOWED_HOSTS.includes(parsed.hostname); }
    catch { return false; }
}
async function handlerA(req: Request, res: Response) {
    const url = `https://${req.query.host}/api/data`;
    if (!isUrlSafe(url)) return res.status(403).json({ error: "URL not allowed" });
    const response = await fetch(url); const data = await response.json(); res.json(data);
}
async function handlerB(req: Request, res: Response) {
    const url = `https://api.example.com/${req.body.path}`;
    if (!isUrlSafe(url)) return res.status(403).json({ error: "URL not allowed" });
    const response = await fetch(url); const data = await response.json(); res.json(data);
}
