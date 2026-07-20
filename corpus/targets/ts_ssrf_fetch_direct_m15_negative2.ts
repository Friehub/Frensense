// SAFE: .then() chain with URL allowlist set
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);

function fetchUserData(req: Request, res: Response) {
  Promise.resolve(req.query.url).then(url => {
    try { const parsed = new URL(url); if (!ALLOWED_HOSTS.has(parsed.hostname)) return res.status(403).send("Host not allowed"); fetch(url).then(response => response.json()).then(data => res.json(data)); } catch { res.status(400).send("Invalid URL"); }
  });
}
