// SAFE: Uses Prisma ORM methods instead of raw queries
async function handlerA(req: Request, res: Response) {
  const val = req.query.id;
  const user = await prisma.user.findUnique({ where: { id: val } });
  res.json(user);
}
async function handlerB(req: Request, res: Response) {
  const val = req.body.email;
  await prisma.user.update({ where: { id: req.userId }, data: { email: val } });
  res.json({ success: true });
}
