// [frensense]
// observation: User-controlled input is passed directly to eval, allowing arbitrary code execution via destructured object property.
// impact: An attacker can execute arbitrary JavaScript on the server by supplying crafted input
// improvement: Avoid eval; use safer alternatives like Function constructor with sanitization or mathjs

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const result = eval(input); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const result = eval(value); res.json({ result });
    res.json({ ok: true });
}
