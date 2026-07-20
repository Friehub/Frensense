// SAFE: Array element URL validated before fetch
const ALLOWED = new Set(["api.trusted.com"]);

function validateUrl(raw: string): URL {
  const parsed = new URL(raw);
  if (!ALLOWED.has(parsed.hostname)) throw new Error("Not allowed");
  return parsed;
}

async function fetchUserData(req: Request, res: Response) {
  try {
    const urls = [req.query.url];
    validateUrl(urls[0]);
    const response = await fetch(urls[0]);
    res.json(await response.json());
  } catch {
    res.status(403).json({ error: "Invalid URL" });
  }
}
