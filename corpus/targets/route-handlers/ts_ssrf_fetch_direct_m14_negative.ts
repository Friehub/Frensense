// SAFE: Renamed variables with URL validation
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];
function isValidUrl(url: string): boolean { try { const parsed = new URL(url); return ALLOWED_HOSTS.includes(parsed.hostname); } catch { return false; } }

async function fetchUserData(req: Request, res: Response) {
    const userProvidedUrl = req.query.url;
    if (!isValidUrl(userProvidedUrl)) return res.status(403).send("Host not allowed");
    const response = await fetch(userProvidedUrl); const data = await response.json(); res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const requestTarget = req.body.target;
    if (!isValidUrl(requestTarget)) return res.status(403).send("Host not allowed");
    const result = await fetch(requestTarget); const body = await result.text(); res.send(body);
}
