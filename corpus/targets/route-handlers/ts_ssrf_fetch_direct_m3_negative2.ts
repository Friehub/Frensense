// SAFE: Multi-hop with URL validation
const ALLOWED_HOSTS = new Set(["api.trusted.com"]);

function validateUrl(raw: string): URL {
  const parsed = new URL(raw);
  if (!ALLOWED_HOSTS.has(parsed.hostname)) throw new Error("Not allowed");
  return parsed;
}

async function fetchUserData(req: Request, res: Response) {
  try {
    const a = req.query.url;
    const b = a;
    const url = validateUrl(b);
    const response = await fetch(url);
    res.json(await response.json());
  } catch {
    res.status(403).json({ error: "Invalid URL" });
  }
}
