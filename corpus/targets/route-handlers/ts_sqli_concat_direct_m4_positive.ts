// [frensense]
// observation: User-controlled input is passed through a helper function that builds a SQL query via string concatenation, bypassing parameterized queries.
// impact: An attacker can perform SQL injection by supplying crafted input to the helper function.
// improvement: Use parameterized queries with placeholders in the helper function instead of string concatenation.

function buildUserQuery(id: string): string {
    return "SELECT * FROM users WHERE id = '" + id + "'";
}

function buildDeleteQuery(orderId: string): string {
    return "DELETE FROM orders WHERE id = '" + orderId + "'";
}

async function getUserById(req: Request, res: Response) {
    const query = buildUserQuery(req.params.id);
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const sql = buildDeleteQuery(req.body.orderId);
    await db.query(sql);
    res.json({ success: true });
}
