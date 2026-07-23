// SAFE: Uses Knex query builder instead of raw for safe parameterized queries
async function searchOrders(req: Request, res: Response) {
  const status = req.query.status;
  const results = await knex("orders").where("status", status);
  res.json(results);
}
