// SAFE: Async path with $queryRaw tagged template
async function getId(req: any): Promise<string> { return req.query.id; }
async function getEmail(req: any): Promise<string> { return req.body.email; }

async function getUser(req: Request, res: Response) {
    const userId = await getId(req);
    const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${userId}`;
    res.json(user);
}

async function updateEmail(req: Request, res: Response) {
    const newEmail = await getEmail(req);
    await prisma.$queryRaw`UPDATE users SET email = ${newEmail} WHERE id = ${req.userId}`;
    res.json({ success: true });
}
