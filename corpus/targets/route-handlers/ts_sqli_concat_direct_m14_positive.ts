// [frensense]
// observation: User-controlled input is concatenated into a SQL query without parameterization with renamed variables.
// impact: An attacker can perform SQL injection.
// improvement: Use parameterized queries

async function getUserById(req: Request, res: Response) {
    const userInput = req.params.id;
    const rawSql = "SELECT * FROM users WHERE id = '" + userInput + "'";
    const result = await db.query(rawSql); res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderIdentifier = req.body.orderId;
    await db.query("DELETE FROM orders WHERE id = '" + orderIdentifier + "'");
    res.json({ success: true });
}
