// SAFE: Conditional branch with $queryRaw tagged template
async function getUser(req: Request, res: Response) {
    if (req.query.id) {
        const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${req.query.id}`;
        res.json(user);
    } else { res.status(400).send("Missing id"); }
}

async function updateEmail(req: Request, res: Response) {
    if (req.body.email && req.body.email.length > 0) {
        await prisma.$queryRaw`UPDATE users SET email = ${req.body.email} WHERE id = ${req.userId}`;
        res.json({ success: true });
    }
}
