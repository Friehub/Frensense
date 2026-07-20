// SAFE: Uses Knex query builder instead of raw
async function handlerA(req: Request, res: Response) {
  const results = await knex("orders").where("status", req.query.status);
  res.json(results);
}
async function handlerB(req: Request, res: Response) {
  const results = await knex("items").where("id", req.body.id);
  res.json(results);
}
