// SAFE: Object property URL validated with URL constructor
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];

async function fetchUserData(req: Request, res: Response) {
  const cfg = { url: req.query.url };
  try {
    const parsed = new URL(cfg.url);
    if (!ALLOWED_HOSTS.includes(parsed.hostname)) return res.status(403).send("Host not allowed");
    parsed.pathname = "/safe" + parsed.pathname;
    const response = await fetch(parsed.toString());
    const data = await response.json();
    res.json(data);
  } catch { res.status(400).send("Invalid URL"); }
}
