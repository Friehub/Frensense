// [frensense]
// observation: User-controlled input flows into a Prisma raw query without parameterization via a template literal interpolation.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use Prisma tagged template $queryRaw which enforces parameterization
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function handlerA(req: Request, res: Response) {
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${req.query.id}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${req.body.email}' WHERE id = ${req.userId}`);
    res.json({ ok: true });
}
