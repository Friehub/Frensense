// SAFE: Conditional branch with $queryRaw tagged template
async function getUser(req: Request, res: Response) {
    if (req.query.id) {
        const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${req.query.id}`;
        res.json(user);
    }
}
