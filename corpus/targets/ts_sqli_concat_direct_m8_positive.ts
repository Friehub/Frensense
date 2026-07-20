// [frensense]
// observation: User-controlled input is accessed through an array element and concatenated into a SQL query without parameterization.
// impact: An attacker can perform SQL injection by supplying crafted input through array-indexed parameters.
// improvement: Use parameterized queries with placeholders instead of string concatenation.

async function getUserById(req: Request, res: Response) {
    const ids = [req.params.id];
    const query = "SELECT * FROM users WHERE id = '" + ids[0] + "'";
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderIds = [req.body.orderId];
    await db.query("DELETE FROM orders WHERE id = '" + orderIds[0] + "'");
    res.json({ success: true });
}
