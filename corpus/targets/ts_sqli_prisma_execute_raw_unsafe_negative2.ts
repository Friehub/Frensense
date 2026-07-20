// SAFE: Uses Prisma's built-in delete method instead of raw SQL
async function removeUser(req: Request, res: Response) {
  const userId = req.body.userId;
  await prisma.user.delete({ where: { id: userId } });
  res.json({ deleted: true });
}
