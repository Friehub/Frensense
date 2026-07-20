// [frensense]
// observation: User-controlled input flows into a Prisma raw query without parameterization via a template literal interpolation.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use Prisma tagged template $queryRaw which enforces parameterization

async function handlerA(req: Request, res: Response) {
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${req.query.id}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${req.body.email}' WHERE id = ${req.userId}`);
    res.json({ ok: true });
}
