// SAFE: Implements safe alternative
// SAFE: Uses Prisma tagged template $queryRaw which enforces parameterized queries
async function handlerA(req: Request, res: Response) {
    const val = req.query.id;
    const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${val}`;
    res.json({ ok: true });
}
async function handlerB(req: Request, res: Response) {
    const val = req.body.email;
    await prisma.$executeRaw`UPDATE users SET email = ${val} WHERE id = ${req.userId}`;
    res.json({ success: true });
}
