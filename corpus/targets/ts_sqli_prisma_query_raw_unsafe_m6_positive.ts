// [frensense]
// observation: User-controlled input flows into a Prisma raw query without parameterization via string concatenation.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use Prisma tagged template $queryRaw which enforces parameterization

async function handlerA(req: Request, res: Response) {
    const q = "SELECT * FROM users WHERE id = '" + req.query.id + "'"; const user = await prisma.$queryRawUnsafe(q);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const q = "UPDATE users SET email = '" + req.body.email + "' WHERE id = " + req.userId; await prisma.$queryRawUnsafe(q);
    res.json({ ok: true });
}
