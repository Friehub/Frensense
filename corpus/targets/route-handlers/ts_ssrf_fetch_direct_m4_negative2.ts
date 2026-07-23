// SAFE: Helper validates URL against allowlist
const ALLOWED = new Set(["api.trusted.com"]);

function safeFetchUrl(raw: string): Promise<Response> {
  const parsed = new URL(raw);
  if (!ALLOWED.has(parsed.hostname)) return Promise.reject("Not allowed");
  if (parsed.protocol !== "https:") return Promise.reject("HTTPS only");
  return fetch(raw, { signal: AbortSignal.timeout(5000) });
}

async function fetchUserData(req: Request, res: Response) {
  try {
    const response = await safeFetchUrl(req.query.url);
    res.json(await response.json());
  } catch {
    res.status(403).json({ error: "Invalid URL" });
  }
}
