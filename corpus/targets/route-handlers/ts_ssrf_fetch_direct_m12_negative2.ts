// SAFE: Try-catch with URL allowlist
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);

async function fetchUserData(req: Request, res: Response) {
  try {
    const parsed = new URL(req.query.url);
    if (!ALLOWED_HOSTS.has(parsed.hostname)) return res.status(403).send("Host not allowed");
    const response = await fetch(req.query.url); const data = await response.json(); res.json(data);
  } catch (err) { res.status(500).json({ error: err.message }); }
}
