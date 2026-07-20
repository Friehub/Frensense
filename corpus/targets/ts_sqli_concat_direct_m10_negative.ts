// SAFE: Async path with parameterized query
async function getId(req: any): Promise<string> { return req.params.id; }
async function getOrderId(req: any): Promise<string> { return req.body.orderId; }

async function getUserById(req: Request, res: Response) {
    const userId = await getId(req);
    const result = await db.query("SELECT * FROM users WHERE id = $1", [userId]);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderId = await getOrderId(req);
    await db.query("DELETE FROM orders WHERE id = $1", [orderId]);
    res.json({ success: true });
}
