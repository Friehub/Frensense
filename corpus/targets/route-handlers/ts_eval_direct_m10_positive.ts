// [frensense]
// observation: User-controlled input is passed directly to eval across an async/await boundary.
// impact: An attacker can execute arbitrary JavaScript.
// improvement: Avoid eval; use mathjs or JSON.parse

async function getExpr(req: any): Promise<string> { return req.body.expression; }
async function getCode(req: any): Promise<string> { return req.query.code; }

async function handlerA(req: Request, res: Response) {
    const val = await getExpr(req); const result = eval(val); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = await getCode(req); const result = eval(val); res.json({ result });
    res.json({ ok: true });
}
