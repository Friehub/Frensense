// [frensense]
// observation: User-controlled input is interpolated into raw SQL via $queryRawUnsafe with renamed variables.
// impact: An attacker can perform SQL injection.
// improvement: Use $queryRaw tagged template literals
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getUser(req: Request, res: Response) {
    const userSuppliedId = req.query.id;
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${userSuppliedId}'`);
    res.json(user);
}

async function updateEmail(req: Request, res: Response) {
    const newEmailAddress = req.body.email;
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${newEmailAddress}' WHERE id = ${req.userId}`);
    res.json({ success: true });
}
