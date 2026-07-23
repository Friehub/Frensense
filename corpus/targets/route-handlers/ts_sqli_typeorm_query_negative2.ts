// SAFE: Uses TypeORM repository find method instead of raw query
async function runQuery(req: Request, res: Response) {
  const input = req.body.sql;
  const result = await AppDataSource.getRepository(User).find({ where: { name: input } });
  res.json(result);
}
