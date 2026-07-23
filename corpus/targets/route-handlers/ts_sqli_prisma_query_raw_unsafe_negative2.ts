// SAFE: Uses Prisma's findUnique and update methods instead of raw SQL
async function getUser(req: Request, res: Response) {
  const userId = req.query.id;
  const user = await prisma.user.findUnique({ where: { id: userId } });
  res.json(user);
}

async function updateEmail(req: Request, res: Response) {
  const newEmail = req.body.email;
  await prisma.user.update({ where: { id: req.userId }, data: { email: newEmail } });
  res.json({ success: true });
}
