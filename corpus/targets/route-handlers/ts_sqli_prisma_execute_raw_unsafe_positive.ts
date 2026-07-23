async function removeUser(req: Request, res: Response) {
    const userId = req.body.userId;
    await prisma.$executeRawUnsafe(`DELETE FROM users WHERE id = '${userId}'`);
    res.json({ deleted: true });
}
