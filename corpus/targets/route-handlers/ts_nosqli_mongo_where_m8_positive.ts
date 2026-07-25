// [frensense]
// observation: User-controlled string is passed directly to the $where operator in a MongoDB query via an array element access.
// impact: An attacker can inject JavaScript code that extracts sensitive data via the $where clause
// improvement: Remove the $where clause entirely or validate input against an allowlist of safe expressions
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

async function handlerA(req: Request, res: Response) {
    const arr = [req.body.condition];
    const users = await db.collection("users").find({ $where: arr[0] }).toArray(); res.json(users);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [req.query.filter];
    const users = await db.collection("sessions").find({ $where: `this.role === "admin" && ${items[0]}` }).toArray(); res.json(users);
    res.json({ ok: true });
}
