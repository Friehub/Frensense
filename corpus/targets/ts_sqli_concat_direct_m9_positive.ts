// [frensense]
// observation: User-controlled input is concatenated into a SQL query without parameterization through an object property.
// impact: An attacker can perform SQL injection by crafting input that breaks out of the string literal.
// improvement: Use parameterized queries or an ORM with bound parameters instead of string concatenation

async function getUserById(req: Request, res: Response) {
    const cfg = { id: req.params.id };
    const query = "SELECT * FROM users WHERE id = '" + cfg.id + "'";
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const params = { orderId: req.body.orderId };
    await db.query("DELETE FROM orders WHERE id = '" + params.orderId + "'");
    res.json({ success: true });
}
