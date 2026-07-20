// [frensense]
// observation: User-controlled string is passed directly to the $where operator in a MongoDB query through an intermediate variable.
// impact: An attacker can inject JavaScript code that extracts sensitive data via the $where clause
// improvement: Remove the $where clause entirely or validate input against an allowlist of safe expressions

async function handlerA(req: Request, res: Response) {
    const val = req.body.condition;
    const users = await db.collection("users").find({ $where: val }).toArray(); res.json(users);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = req.query.filter;
    const users = await db.collection("sessions").find({ $where: `this.role === "admin" && ${val}` }).toArray(); res.json(users);
    res.json({ ok: true });
}
