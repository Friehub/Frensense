// [frensense]
// observation: User-controlled string is passed directly to the $where operator in a MongoDB query through multiple variable assignments.
// impact: An attacker can inject JavaScript code that extracts sensitive data via the $where clause
// improvement: Remove the $where clause entirely or validate input against an allowlist of safe expressions
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

async function handlerA(req: Request, res: Response) {
    const a = req.body.condition;
    const b = a;
    const users = await db.collection("users").find({ $where: b }).toArray(); res.json(users);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.query.filter;
    const y = x;
    const z = y;
    const users = await db.collection("sessions").find({ $where: `this.role === "admin" && ${z}` }).toArray(); res.json(users);
    res.json({ ok: true });
}
