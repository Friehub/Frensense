// [frensense]
// observation: User-controlled input is interpolated into raw SQL via $queryRawUnsafe inside a try-catch block.
// impact: An attacker can perform SQL injection, with errors silently caught.
// improvement: Use $queryRaw tagged template literals
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

async function getUser(req: Request, res: Response) {
    try { const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${req.query.id}'`); res.json(user); } catch (err) { console.error(err); }
}

async function updateEmail(req: Request, res: Response) {
    try { await prisma.$queryRawUnsafe(`UPDATE users SET email = '${req.body.email}' WHERE id = ${req.userId}`); res.json({ success: true }); } catch {}
}
