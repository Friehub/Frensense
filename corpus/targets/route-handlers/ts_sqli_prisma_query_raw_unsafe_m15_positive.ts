// [frensense]
// observation: User-controlled input is interpolated into raw SQL via $queryRawUnsafe via a promise .then() chain.
// impact: An attacker can perform SQL injection.
// improvement: Use $queryRaw tagged template literals

function getUser(req: Request, res: Response) {
    Promise.resolve(req.query.id).then(userId => {
        prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${userId}'`).then(user => res.json(user));
    });
}

function updateEmail(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.email)).then(newEmail => {
        prisma.$queryRawUnsafe(`UPDATE users SET email = '${newEmail}' WHERE id = ${req.userId}`).then(() => res.json({ success: true }));
    });
}
