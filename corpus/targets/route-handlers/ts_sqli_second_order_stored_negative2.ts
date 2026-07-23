// SAFE: Uses parameterized queries for both insert and select operations
async function handleLogin(req: Request, res: Response) {
  const username = req.body.username;
  await db.query("INSERT INTO audit_log (username) VALUES ($1)", [username]);
  const result = await db.query("SELECT * FROM users WHERE username = $1", [username]);
  res.json(result.rows[0]);
}
