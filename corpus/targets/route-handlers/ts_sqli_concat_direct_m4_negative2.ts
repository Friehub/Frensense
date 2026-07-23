// SAFE: Helper function uses parameterized query internally
function queryUser(id: string) {
    return pool.query("SELECT * FROM users WHERE id = $1", [id]);
}

function deleteOrderById(orderId: string) {
    return pool.query("DELETE FROM orders WHERE id = $1", [orderId]);
}

async function getUserById(req: Request, res: Response) {
  const result = await queryUser(req.params.id);
  res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
  await deleteOrderById(req.body.orderId);
  res.json({ success: true });
}
