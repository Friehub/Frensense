// [frensense]
// observation: User-controlled input is passed directly to eval, allowing arbitrary code execution via an array element access.
// impact: An attacker can execute arbitrary JavaScript on the server by supplying crafted input
// improvement: Avoid eval; use safer alternatives like Function constructor with sanitization or mathjs

async function handlerA(req: Request, res: Response) {
    const arr = [req.body.expression];
    const result = eval(arr[0]); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [req.query.code];
    const result = eval(items[0]); res.json({ result });
    res.json({ ok: true });
}
