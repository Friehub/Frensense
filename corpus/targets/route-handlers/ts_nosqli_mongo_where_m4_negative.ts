// SAFE: Implements safe alternative
// SAFE: Removed $where operator and used standard MongoDB query operators
async function handlerA(req: Request, res: Response) {
    const { role, status } = req.body;
    const query = {};
    if (role) query.role = role;
    if (status) query.status = status;
    const users = await db.collection("users").find(query).toArray();
    res.json(users);
}
async function handlerB(req: Request, res: Response) {
    const result = await db.collection("sessions").find({ role: "admin" }).toArray();
    res.json(result);
}
