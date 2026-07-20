// SAFE: Try-catch with $queryRaw tagged template
async function getUser(req: Request, res: Response) {
  try { const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${req.query.id}`; res.json(user); } catch (err) { res.status(500).json({ error: err.message }); }
}
