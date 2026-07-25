// [frensense]
// observation: User-controlled input flows into a Prisma raw query without parameterization via an array element access.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use Prisma tagged template $queryRaw which enforces parameterization
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function handlerA(req: Request, res: Response) {
    const arr = [req.query.id];
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${arr[0]}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [req.body.email];
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${items[0]}' WHERE id = ${req.userId}`);
    res.json({ ok: true });
}
