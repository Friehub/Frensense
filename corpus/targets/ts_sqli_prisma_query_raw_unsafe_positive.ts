async function getUser(req: Request, res: Response) {
    const userId = req.query.id;
    const user = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${userId}'`);
    res.json(user);
}

async function updateEmail(req: Request, res: Response) {
    const newEmail = req.body.email;
    await prisma.$queryRawUnsafe(`UPDATE users SET email = '${newEmail}' WHERE id = ${req.userId}`);
    res.json({ success: true });
}
