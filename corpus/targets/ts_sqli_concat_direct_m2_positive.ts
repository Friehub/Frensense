// [frensense]
// observation: User-controlled input flows through an intermediate variable into a SQL query built via string concatenation without parameterization.
// impact: An attacker can perform SQL injection by supplying crafted input in the URL parameter.
// improvement: Use parameterized queries with placeholders instead of string concatenation.

async function getUserById(req: Request, res: Response) {
    const userId = req.params.id;
    const query = "SELECT * FROM users WHERE id = '" + userId + "'";
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderId = req.body.orderId;
    const sql = "DELETE FROM orders WHERE id = '" + orderId + "'";
    await db.query(sql);
    res.json({ success: true });
}
