// SAFE: Template literal not used for SQL — uses parameterized binding
async function getUserById(req: Request, res: Response) {
  const result = await pool.query("SELECT * FROM users WHERE id = $1", [req.params.id]);
  res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
  await pool.query("DELETE FROM orders WHERE id = $1", [req.body.orderId]);
  res.json({ success: true });
}
