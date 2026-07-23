async function searchOrders(req: Request, res: Response) {
    const status = req.query.status;
    const results = await knex.raw(`SELECT * FROM orders WHERE status = '${status}'`);
    res.json(results.rows);
}
