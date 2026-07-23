// SAFE: Uses TypeORM FindManyOptions with where clause instead of QueryBuilder
async function searchUsers(req: Request, res: Response) {
  const name = req.query.name;
  const users = await AppDataSource.getRepository(User).find({
    where: { name }
  });
  res.json(users);
}
