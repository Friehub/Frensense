// [frensense]
// observation: User-controlled string is passed directly to the $where operator in a MongoDB query through a helper function.
// impact: An attacker can inject JavaScript code that extracts sensitive data via the $where clause
// improvement: Remove the $where clause entirely or validate input against an allowlist of safe expressions

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(req.body.condition);
    const users = await db.collection("users").find({ $where: val }).toArray(); res.json(users);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(req.query.filter);
    const users = await db.collection("sessions").find({ $where: `this.role === "admin" && ${val}` }).toArray(); res.json(users);
    res.json({ ok: true });
}
