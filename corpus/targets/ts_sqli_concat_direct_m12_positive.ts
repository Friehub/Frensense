// [frensense]
// observation: User-controlled input is concatenated into a SQL query without parameterization inside a try-catch block.
// impact: An attacker can perform SQL injection, with errors silently caught.
// improvement: Use parameterized queries

async function getUserById(req: Request, res: Response) {
    try {
        const query = "SELECT * FROM users WHERE id = '" + req.params.id + "'";
        const result = await db.query(query); res.json(result.rows[0]);
    } catch (err) { console.error(err); }
}

async function deleteOrder(req: Request, res: Response) {
    try {
        await db.query("DELETE FROM orders WHERE id = '" + req.body.orderId + "'");
        res.json({ success: true });
    } catch {}
}
