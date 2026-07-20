// SAFE: Uses Sequelize model findAll instead of raw query
async function findUsers(req: Request, res: Response) {
  const name = req.query.name;
  const results = await User.findAll({
    where: { name: { [Op.like]: `%${name}%` } }
  });
  res.json(results);
}
