// SAFE: Object property via $queryRaw tagged template
async function getUser(req: Request, res: Response) {
    const params = { id: req.query.id };
    const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${params.id}`;
    res.json(user);
}

async function updateEmail(req: Request, res: Response) {
    const data = { email: req.body.email };
    await prisma.$queryRaw`UPDATE users SET email = ${data.email} WHERE id = ${req.userId}`;
    res.json({ success: true });
}
