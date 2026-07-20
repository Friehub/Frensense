// [frensense]
// observation: User-controlled input flows into a Prisma raw query without parameterization through an intermediate variable.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use Prisma tagged template $queryRaw which enforces parameterization

async function handlerA(req: Request, res: Response) {
    const val = req.query.id;
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${val}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = req.body.email;
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${val}' WHERE id = ${req.userId}`);
    res.json({ ok: true });
}
