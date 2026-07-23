// SAFE: Try-catch with $queryRaw tagged template
async function getUser(req: Request, res: Response) {
    try { const user = await prisma.$queryRaw`SELECT * FROM users WHERE id = ${req.query.id}`; res.json(user); } catch (err) { console.error(err); res.status(500).send("Error"); }
}

async function updateEmail(req: Request, res: Response) {
    try { await prisma.$queryRaw`UPDATE users SET email = ${req.body.email} WHERE id = ${req.userId}`; res.json({ success: true }); } catch (err) { console.error(err); res.status(500).send("Error"); }
}
