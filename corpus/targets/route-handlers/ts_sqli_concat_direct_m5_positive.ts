// [frensense]
// observation: User-controlled input is interpolated into a SQL query via a template literal, enabling SQL injection through string interpolation.
// impact: An attacker can perform SQL injection by supplying crafted input in the URL parameter or request body.
// improvement: Use parameterized queries with placeholders instead of template literal interpolation.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getUserById(req: Request, res: Response) {
    const query = `SELECT * FROM users WHERE id = '${req.params.id}'`;
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    await db.query(`DELETE FROM orders WHERE id = '${req.body.orderId}'`);
    res.json({ success: true });
}
