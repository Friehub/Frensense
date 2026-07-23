// SAFE: Destructured URL validated against allowlist
const ALLOWED = new Set(["api.trusted.com"]);

function validateUrl(raw: string): URL {
  const parsed = new URL(raw);
  if (!ALLOWED.has(parsed.hostname)) throw new Error("Not allowed");
  return parsed;
}

async function fetchUserData(req: Request, res: Response) {
  try {
    const { url } = req.query;
    validateUrl(url);
    const response = await fetch(url);
    res.json(await response.json());
  } catch {
    res.status(403).json({ error: "Invalid URL" });
  }
}
