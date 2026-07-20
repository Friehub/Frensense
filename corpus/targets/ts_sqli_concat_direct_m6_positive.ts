// [frensense]
// observation: User-controlled input is concatenated directly into the SQL query string at the call site without parameterization.
// impact: An attacker can perform SQL injection by supplying crafted input that alters the query structure.
// improvement: Use parameterized queries with placeholders instead of string concatenation.

async function getUserById(req: Request, res: Response) {
    const result = await db.query("SELECT * FROM users WHERE id = '" + req.params.id + "'");
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    await db.query("DELETE FROM orders WHERE id = '" + req.body.orderId + "'");
    res.json({ success: true });
}
