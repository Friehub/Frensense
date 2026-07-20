async function getUserById(req: Request, res: Response) {
    const result = await db.query("SELECT * FROM users WHERE id = $1", [req.params.id]);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    await db.query("DELETE FROM orders WHERE id = $1", [req.body.orderId]);
    res.json({ success: true });
}
