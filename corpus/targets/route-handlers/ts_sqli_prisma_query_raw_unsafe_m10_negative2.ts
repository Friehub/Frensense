// SAFE: Async path with $queryRaw tagged template
async function getId(req: any): Promise<string> { return req.query.id; }
async function getUser(req: Request, res: Response) {
    const userId = await getId(req);
    const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${userId}`;
    res.json(user);
}
