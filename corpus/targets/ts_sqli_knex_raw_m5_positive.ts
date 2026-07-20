// [frensense]
// observation: User-controlled input flows into a Knex raw query without parameterization via a template literal interpolation.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use parameterized Knex raw queries with ? placeholders or the query builder

async function handlerA(req: Request, res: Response) {
    const results = await knex.raw(`SELECT * FROM orders WHERE status = '${req.query.status}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const results = await knex.raw(`SELECT * FROM items WHERE id = '${req.body.id}'`);
    res.json({ ok: true });
}
