// SAFE: Async path with URL validation
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];
function isValidUrl(url: string): boolean {
    try { const parsed = new URL(url); return ALLOWED_HOSTS.includes(parsed.hostname); } catch { return false; }
}

async function getUrl(req: any): Promise<string> {
    const url = req.query.url;
    if (!isValidUrl(url)) throw new Error("Not allowed"); return url;
}

async function getTarget(req: any): Promise<string> {
    const t = req.body.target; if (!isValidUrl(t)) throw new Error("Not allowed"); return t;
}

async function fetchUserData(req: Request, res: Response) {
    try { const url = await getUrl(req); const response = await fetch(url); const data = await response.json(); res.json(data); } catch { res.status(403).send("Host not allowed"); }
}

async function proxyRequest(req: Request, res: Response) {
    try { const target = await getTarget(req); const result = await fetch(target); const body = await result.text(); res.send(body); } catch { res.status(403).send("Host not allowed"); }
}
