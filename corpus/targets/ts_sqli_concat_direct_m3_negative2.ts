// SAFE: Multi-hop assignment with parameterized query
async function getUserById(req: Request, res: Response) {
  const raw = req.params.id;
  const userId = raw;
  const result = await pool.query("SELECT * FROM users WHERE id = $1", [userId]);
  res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
  const input = req.body.orderId;
  const orderId = input;
  const val = orderId;
  await pool.query("DELETE FROM orders WHERE id = $1", [val]);
  res.json({ success: true });
}
