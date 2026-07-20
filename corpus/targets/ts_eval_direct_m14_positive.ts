// [frensense]
// observation: User-controlled input is passed directly to eval with renamed variables.
// impact: An attacker can execute arbitrary JavaScript.
// improvement: Avoid eval; use mathjs or JSON.parse

async function handlerA(req: Request, res: Response) {
    const userExpression = req.body.expression;
    const result = eval(userExpression); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const codeSnippet = req.query.code;
    const result = eval(codeSnippet); res.json({ result });
    res.json({ ok: true });
}
