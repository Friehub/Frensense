// [frensense]
// observation: User-controlled input flows into a Knex raw query without parameterization through multiple variable assignments.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use parameterized Knex raw queries with ? placeholders or the query builder
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function handlerA(req: Request, res: Response) {
    const a = req.query.status;
    const b = a;
    const results = await knex.raw(`SELECT * FROM orders WHERE status = '${b}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.body.id;
    const y = x;
    const z = y;
    const results = await knex.raw(`SELECT * FROM items WHERE id = '${z}'`);
    res.json({ ok: true });
}
