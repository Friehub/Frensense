// SAFE: Renamed variables with URL allowlist set
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);

async function fetchUserData(req: Request, res: Response) {
  const userProvidedUrl = req.query.url;
  try { const parsed = new URL(userProvidedUrl); if (!ALLOWED_HOSTS.has(parsed.hostname)) return res.status(403).send("Host not allowed"); const response = await fetch(userProvidedUrl); const data = await response.json(); res.json(data); } catch { res.status(400).send("Invalid URL"); }
}
