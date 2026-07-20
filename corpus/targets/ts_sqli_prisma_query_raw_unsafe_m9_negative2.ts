// SAFE: Object property via $queryRaw tagged template
async function getUser(req: Request, res: Response) {
  const params = { id: req.query.id };
  const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${params.id}`;
  res.json(user);
}
