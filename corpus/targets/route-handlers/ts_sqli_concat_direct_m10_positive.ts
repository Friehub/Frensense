// [frensense]
// observation: User-controlled input is concatenated into a SQL query without parameterization across an async/await boundary.
// impact: An attacker can perform SQL injection.
// improvement: Use parameterized queries
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getId(req: any): Promise<string> { return req.params.id; }
async function getOrderId(req: any): Promise<string> { return req.body.orderId; }

async function getUserById(req: Request, res: Response) {
    const userId = await getId(req);
    const query = "SELECT * FROM users WHERE id = '" + userId + "'";
    const result = await db.query(query);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const orderId = await getOrderId(req);
    await db.query("DELETE FROM orders WHERE id = '" + orderId + "'");
    res.json({ success: true });
}
