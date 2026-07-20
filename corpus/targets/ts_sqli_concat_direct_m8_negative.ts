async function getUserById(req: Request, res: Response) {
    const ids = [req.params.id];
    const result = await db.query("SELECT * FROM users WHERE id = $1", [ids[0]]);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderIds = [req.body.orderId];
    await db.query("DELETE FROM orders WHERE id = $1", [orderIds[0]]);
    res.json({ success: true });
}
