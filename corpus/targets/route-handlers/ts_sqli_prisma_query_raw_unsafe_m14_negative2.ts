// SAFE: Renamed variables with $queryRaw tagged template
async function getUser(req: Request, res: Response) {
    const userSuppliedId = req.query.id;
    const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${userSuppliedId}`;
    res.json(user);
}
