// [frensense]
// observation: User-controlled input is destructured from an object and then concatenated into a SQL query without parameterization.
// impact: An attacker can perform SQL injection by crafting the destructured input value.
// improvement: Use parameterized queries with placeholders instead of string concatenation.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getUserById(req: Request, res: Response) {
    const { id } = req.params;
    const query = "SELECT * FROM users WHERE id = '" + id + "'";
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const { orderId } = req.body;
    await db.query("DELETE FROM orders WHERE id = '" + orderId + "'");
    res.json({ success: true });
}
