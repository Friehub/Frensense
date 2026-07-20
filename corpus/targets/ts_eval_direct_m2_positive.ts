// [frensense]
// observation: User-controlled input is passed directly to eval, allowing arbitrary code execution through an intermediate variable.
// impact: An attacker can execute arbitrary JavaScript on the server by supplying crafted input
// improvement: Avoid eval; use safer alternatives like Function constructor with sanitization or mathjs

async function handlerA(req: Request, res: Response) {
    const val = req.body.expression;
    const result = eval(val); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = req.query.code;
    const result = eval(val); res.json({ result });
    res.json({ ok: true });
}
