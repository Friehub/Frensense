// SAFE: Try-catch with URL validation
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];
function isValidUrl(url: string): boolean {
    try { const parsed = new URL(url); return ALLOWED_HOSTS.includes(parsed.hostname); } catch { return false; }
}

async function fetchUserData(req: Request, res: Response) {
    try { if (!isValidUrl(req.query.url)) return res.status(403).send("Host not allowed"); const response = await fetch(req.query.url); const data = await response.json(); res.json(data); } catch (err) { console.error(err); res.status(500).send("Error"); }
}

async function proxyRequest(req: Request, res: Response) {
    try { if (!isValidUrl(req.body.target)) return res.status(403).send("Host not allowed"); const result = await fetch(req.body.target); const body = await result.text(); res.send(body); } catch (err) { console.error(err); res.status(500).send("Error"); }
}
