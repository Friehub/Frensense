// [frensense]
// observation: User-controlled input is passed directly to eval inside a try-catch block.
// impact: An attacker can execute arbitrary JavaScript, with errors silently caught.
// improvement: Avoid eval; use mathjs or JSON.parse

async function handlerA(req: Request, res: Response) {
    try { const result = eval(req.body.expression); res.json({ result }); } catch (err) { console.error(err); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    try { const result = eval(req.query.code); res.json({ result }); } catch {}
    res.json({ ok: true });
}
