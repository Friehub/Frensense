// [frensense]
// observation: User-controlled input is passed directly to eval inside a conditional block on the tainted branch.
// impact: An attacker can execute arbitrary JavaScript.
// improvement: Avoid eval; use mathjs or JSON.parse
// cwe: CWE-95
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

async function handlerA(req: Request, res: Response) {
    if (req.body.expression) {
        const result = eval(req.body.expression); res.json({ result });
    } else { res.json({ error: "No expression" }); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    if (req.query.code && req.query.code.length > 0) {
        const result = eval(req.query.code); res.json({ result });
    }
    res.json({ ok: true });
}
