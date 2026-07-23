// SAFE: Try-catch with parameterized query (alternate)
async function getUserById(req: Request, res: Response) {
  try { const result = await db.query("SELECT * FROM users WHERE id = $1", [req.params.id]); res.json(result.rows[0]); } catch (err) { res.status(500).json({ error: err.message }); }
}
