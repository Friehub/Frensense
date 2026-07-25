// [frensense]
// observation: User-controlled input flows into a Prisma raw query without parameterization via string concatenation.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use Prisma tagged template $queryRaw which enforces parameterization
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function handlerA(req: Request, res: Response) {
    const q = "SELECT * FROM users WHERE id = '" + req.query.id + "'"; const user = await prisma.$queryRawUnsafe(q);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const q = "UPDATE users SET email = '" + req.body.email + "' WHERE id = " + req.userId; await prisma.$queryRawUnsafe(q);
    res.json({ ok: true });
}
