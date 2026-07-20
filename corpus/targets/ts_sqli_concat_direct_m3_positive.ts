// [frensense]
// observation: User-controlled input flows through multiple intermediate variable assignments before reaching a SQL query built via string concatenation.
// impact: An attacker can perform SQL injection by supplying crafted input in the request body or parameters.
// improvement: Use parameterized queries with placeholders instead of string concatenation.

async function getUserById(req: Request, res: Response) {
    const raw = req.params.id;
    const userId = raw;
    const query = "SELECT * FROM users WHERE id = '" + userId + "'";
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const input = req.body.orderId;
    const orderId = input;
    const val = orderId;
    await db.query("DELETE FROM orders WHERE id = '" + val + "'");
    res.json({ success: true });
}
