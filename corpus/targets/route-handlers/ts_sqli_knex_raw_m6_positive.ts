// [frensense]
// observation: User-controlled input flows into a Knex raw query without parameterization via string concatenation.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use parameterized Knex raw queries with ? placeholders or the query builder
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function handlerA(req: Request, res: Response) {
    const q = "SELECT * FROM orders WHERE status = '" + req.query.status + "'"; const results = await knex.raw(q);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const q = "SELECT * FROM items WHERE id = '" + req.body.id + "'"; const results = await knex.raw(q);
    res.json({ ok: true });
}
