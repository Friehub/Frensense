// SAFE: Uses Knex query builder to build dynamic conditions safely
async function buildReport(req: Request, res: Response) {
  const query = knex("reports");
  const userId = req.body.userId;
  const status = req.body.status;
  if (userId) query.where("user_id", userId);
  if (status) query.where("status", status);
  const results = await query;
  res.json(results.rows);
}
