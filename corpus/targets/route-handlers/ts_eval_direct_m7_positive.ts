// [frensense]
// observation: User-controlled input is passed directly to eval, allowing arbitrary code execution via destructured object property.
// impact: An attacker can execute arbitrary JavaScript on the server by supplying crafted input
// improvement: Avoid eval; use safer alternatives like Function constructor with sanitization or mathjs
// cwe: CWE-95
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

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
