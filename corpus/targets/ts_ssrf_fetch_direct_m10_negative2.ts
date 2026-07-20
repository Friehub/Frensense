// SAFE: Async path with URL validation using URL constructor
const ALLOWED_HOSTS = new Set(["api.example.com", "data.example.com"]);

async function validateUrl(req: any): Promise<string> {
  const url = req.query.url;
  const parsed = new URL(url);
  if (!ALLOWED_HOSTS.has(parsed.hostname)) throw new Error("Not allowed");
  return url;
}

async function fetchUserData(req: Request, res: Response) {
  try {
    const url = await validateUrl(req);
    const response = await fetch(url); const data = await response.json(); res.json(data);
  } catch { res.status(403).send("Host not allowed"); }
}
