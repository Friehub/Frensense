// [frensense]
// observation: User-controlled input is interpolated into raw SQL via $queryRawUnsafe inside a conditional block on the tainted branch.
// impact: An attacker can perform SQL injection.
// improvement: Use $queryRaw tagged template literals

async function getUser(req: Request, res: Response) {
    if (req.query.id) {
        const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${req.query.id}'`);
        res.json(user);
    } else { res.status(400).send("Missing id"); }
}

async function updateEmail(req: Request, res: Response) {
    if (req.body.email && req.body.email.length > 0) {
        await prisma.$queryRawUnsafe(`UPDATE users SET email = '${req.body.email}' WHERE id = ${req.userId}`);
        res.json({ success: true });
    }
}
