// [frensense]
// observation: User-controlled input flows into a Knex raw query without parameterization via destructured object property.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use parameterized Knex raw queries with ? placeholders or the query builder

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
