// [frensense]
// observation: User-controlled input flows into a Knex raw query without parameterization through a helper function.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use parameterized Knex raw queries with ? placeholders or the query builder

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(req.query.status);
    const results = await knex.raw(`SELECT * FROM orders WHERE status = '${val}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(req.body.id);
    const results = await knex.raw(`SELECT * FROM items WHERE id = '${val}'`);
    res.json({ ok: true });
}
