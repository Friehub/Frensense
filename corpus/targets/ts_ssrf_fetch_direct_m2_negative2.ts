// SAFE: URL validated against allowlist before fetch
const ALLOWED_HOSTS = new Set(["api.trusted.com", "data.internal.com"]);

function validateUrl(raw: string): URL {
  const parsed = new URL(raw);
  if (!ALLOWED_HOSTS.has(parsed.hostname)) throw new Error("Host not allowed");
  if (parsed.protocol !== "https:") throw new Error("Only HTTPS allowed");
  return parsed;
}

async function fetchUserData(req: Request, res: Response) {
  try {
    const url = validateUrl(req.query.url);
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
  } catch {
    res.status(403).json({ error: "Invalid URL" });
  }
}
