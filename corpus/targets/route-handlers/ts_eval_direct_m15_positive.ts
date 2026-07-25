// [frensense]
// observation: User-controlled input is passed directly to eval via a promise .then() chain.
// impact: An attacker can execute arbitrary JavaScript.
// improvement: Avoid eval; use mathjs or JSON.parse
// cwe: CWE-95
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

function handlerA(req: Request, res: Response) {
    Promise.resolve(req.body.expression).then(val => {
        const result = eval(val); res.json({ result });
    });
    res.json({ ok: true });
}

function handlerB(req: Request, res: Response) {
    new Promise(resolve => resolve(req.query.code)).then(val => {
        const result = eval(val); res.json({ result });
    });
    res.json({ ok: true });
}
