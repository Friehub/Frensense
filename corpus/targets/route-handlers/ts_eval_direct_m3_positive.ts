// [frensense]
// observation: User-controlled input is passed directly to eval, allowing arbitrary code execution through multiple variable assignments.
// impact: An attacker can execute arbitrary JavaScript on the server by supplying crafted input
// improvement: Avoid eval; use safer alternatives like Function constructor with sanitization or mathjs

async function handlerA(req: Request, res: Response) {
    const a = req.body.expression;
    const b = a;
    const result = eval(b); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.query.code;
    const y = x;
    const z = y;
    const result = eval(z); res.json({ result });
    res.json({ ok: true });
}
