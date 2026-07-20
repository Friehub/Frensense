// SAFE: Renamed variables with parameterized query
async function getUserById(req: Request, res: Response) {
    const userInput = req.params.id;
    const result = await db.query("SELECT * FROM users WHERE id = $1", [userInput]);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderIdentifier = req.body.orderId;
    await db.query("DELETE FROM orders WHERE id = $1", [orderIdentifier]);
    res.json({ success: true });
}
