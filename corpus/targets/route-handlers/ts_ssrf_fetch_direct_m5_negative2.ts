// SAFE: Template literal URL validated against allowlist
const ALLOWED = new Set(["api.trusted.com", "data.internal.com"]);

function buildSafeUrl(host: string): string {
  if (!ALLOWED.has(host)) throw new Error("Host not allowed");
  return `https://${host}/v1/data`;
}

async function fetchUserData(req: Request, res: Response) {
  try {
    const url = buildSafeUrl(req.query.url);
    const response = await fetch(url);
    res.json(await response.json());
  } catch {
    res.status(403).json({ error: "Invalid host" });
  }
}
