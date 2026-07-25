// [frensense]
// observation: User-controlled input is interpolated into a raw SQL query via Prisma's $queryRawUnsafe without parameterization through an object property.
// impact: An attacker can perform SQL injection by crafting input that breaks out of the template literal.
// improvement: Use $queryRaw with tagged template literals which auto-parameterize, or use Prisma's type-safe query builder
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getUser(req: Request, res: Response) {
    const params = { id: req.query.id };
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${params.id}'`);
    res.json(user);
}

async function updateEmail(req: Request, res: Response) {
    const data = { email: req.body.email };
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${data.email}' WHERE id = ${req.userId}`);
    res.json({ success: true });
}
