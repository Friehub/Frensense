// [frensense]
// observation: User-controlled input is concatenated into a SQL query without parameterization inside a conditional block on the tainted branch.
// impact: An attacker can perform SQL injection.
// improvement: Use parameterized queries
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getUserById(req: Request, res: Response) {
    if (req.params.id) {
        const query = "SELECT * FROM users WHERE id = '" + req.params.id + "'";
        const result = await db.query(query);
        res.json(result.rows[0]);
    } else {
        res.status(400).send("Missing id");
    }
}

async function deleteOrder(req: Request, res: Response) {
    if (req.body.orderId && req.body.orderId.length > 0) {
        await db.query("DELETE FROM orders WHERE id = '" + req.body.orderId + "'");
        res.json({ success: true });
    }
}
