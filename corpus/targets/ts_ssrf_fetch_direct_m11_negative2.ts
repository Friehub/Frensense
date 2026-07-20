// SAFE: Conditional branch with URL allowlist set
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);

async function fetchUserData(req: Request, res: Response) {
  if (req.query.url) {
    try { const parsed = new URL(req.query.url); if (!ALLOWED_HOSTS.has(parsed.hostname)) return res.status(403).send("Host not allowed"); const response = await fetch(req.query.url); const data = await response.json(); res.json(data); } catch { res.status(400).send("Invalid URL"); }
  }
}
