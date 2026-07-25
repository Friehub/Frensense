// [frensense]
// observation: User-controlled input is interpolated into a raw SQL query via $queryRawUnsafe across an async/await boundary.
// impact: An attacker can perform SQL injection.
// improvement: Use $queryRaw with tagged template literals
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getId(req: any): Promise<string> { return req.query.id; }
async function getEmail(req: any): Promise<string> { return req.body.email; }

async function getUser(req: Request, res: Response) {
    const userId = await getId(req);
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${userId}'`);
    res.json(user);
}

async function updateEmail(req: Request, res: Response) {
    const newEmail = await getEmail(req);
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${newEmail}' WHERE id = ${req.userId}`);
    res.json({ success: true });
}
