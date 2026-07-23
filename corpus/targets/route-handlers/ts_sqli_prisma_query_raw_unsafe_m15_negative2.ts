// SAFE: .then() chain with $queryRaw tagged template
function getUser(req: Request, res: Response) {
    Promise.resolve(req.query.id).then(userId => {
        prisma.$queryRaw`SELECT * FROM users WHERE id = ${userId}`.then(user => res.json(user));
    });
}
