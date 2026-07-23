async function getUserById(req: Request, res: Response) {
    const userId = req.params.id;
    const query = "SELECT * FROM users WHERE id = '" + userId + "'";
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderId = req.body.orderId;
    await db.query("DELETE FROM orders WHERE id = '" + orderId + "'");
    res.json({ success: true });
}
