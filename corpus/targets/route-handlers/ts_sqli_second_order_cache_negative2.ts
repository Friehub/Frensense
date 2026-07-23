// SAFE: Sanitizes and validates input before caching, and uses parameterized queries when reading back
function sanitizeSearch(input: string): string {
  return input.replace(/[^a-zA-Z0-9\s]/g, "");
}

async function handleSearch(req: Request, res: Response) {
  const query = sanitizeSearch(req.body.query);
  await redis.set(`search:${req.userId}`, query);
  const cached = await redis.get(`search:${req.userId}`);
  const result = await db.query("SELECT * FROM items WHERE name = $1", [cached]);
  res.json(result.rows);
}
