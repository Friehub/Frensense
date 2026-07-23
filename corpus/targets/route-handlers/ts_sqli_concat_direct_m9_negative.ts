// SAFE: Object property value is parameterized
async function getUserById(req: Request, res: Response) {
    const cfg = { id: req.params.id };
    const result = await db.query("SELECT * FROM users WHERE id = $1", [cfg.id]);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const params = { orderId: req.body.orderId };
    await db.query("DELETE FROM orders WHERE id = $1", [params.orderId]);
    res.json({ success: true });
}
