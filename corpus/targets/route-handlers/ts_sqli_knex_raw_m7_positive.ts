// [frensense]
// observation: User-controlled input flows into a Knex raw query without parameterization via destructured object property.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use parameterized Knex raw queries with ? placeholders or the query builder
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const results = await knex.raw(`SELECT * FROM orders WHERE status = '${input}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const results = await knex.raw(`SELECT * FROM items WHERE id = '${value}'`);
    res.json({ ok: true });
}
