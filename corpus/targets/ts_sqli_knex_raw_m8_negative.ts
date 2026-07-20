// SAFE: Implements safe alternative
// SAFE: Uses Knex raw with ? parameterized placeholders
async function handlerA(req: Request, res: Response) {
    const results = await knex.raw("SELECT * FROM orders WHERE status = ?", [req.query.status]);
    res.json(results.rows);
}
async function handlerB(req: Request, res: Response) {
    const results = await knex.raw("SELECT * FROM items WHERE id = ?", [req.body.id]);
    res.json(results.rows);
}
