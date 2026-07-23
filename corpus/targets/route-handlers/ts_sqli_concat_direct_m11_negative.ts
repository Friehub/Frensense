// SAFE: Conditional branch with parameterized query
async function getUserById(req: Request, res: Response) {
    if (req.params.id) {
        const result = await db.query("SELECT * FROM users WHERE id = $1", [req.params.id]);
        res.json(result.rows[0]);
    } else {
        res.status(400).send("Missing id");
    }
}

async function deleteOrder(req: Request, res: Response) {
    if (req.body.orderId && req.body.orderId.length > 0) {
        await db.query("DELETE FROM orders WHERE id = $1", [req.body.orderId]);
        res.json({ success: true });
    }
}
