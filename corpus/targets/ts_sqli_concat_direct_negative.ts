async function getUserById(req: Request, res: Response) {
    const userId = req.params.id;
    const result = await db.query("SELECT * FROM users WHERE id = $1", [userId]);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderId = req.body.orderId;
    await db.query("DELETE FROM orders WHERE id = $1", [orderId]);
    res.json({ success: true });
}
