// SAFE: .then() chain with $queryRaw tagged template
function getUser(req: Request, res: Response) {
    Promise.resolve(req.query.id).then(userId => {
        prisma.$queryRaw`SELECT * FROM users WHERE id = ${userId}`.then(user => res.json(user));
    });
}

function updateEmail(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.email)).then(newEmail => {
        prisma.$queryRaw`UPDATE users SET email = ${newEmail} WHERE id = ${req.userId}`.then(() => res.json({ success: true }));
    });
}
