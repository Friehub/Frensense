// [frensense]
// observation: User-controlled input flows into a Prisma raw query without parameterization through a helper function.
// impact: An attacker can perform SQL injection by supplying crafted input
// improvement: Use Prisma tagged template $queryRaw which enforces parameterization
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(req.query.id);
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${val}'`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(req.body.email);
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${val}' WHERE id = ${req.userId}`);
    res.json({ ok: true });
}
