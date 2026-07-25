// [frensense]
// observation: User-controlled input flows into a Knex raw query without parameterization through an intermediate variable.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use parameterized Knex raw queries with ? placeholders or the query builder
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function handlerA(req: Request, res: Response) {
    const val = req.query.status;
    const results = await knex.raw(`SELECT * FROM orders WHERE status = '${val}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = req.body.id;
    const results = await knex.raw(`SELECT * FROM items WHERE id = '${val}'`);
    res.json({ ok: true });
}
