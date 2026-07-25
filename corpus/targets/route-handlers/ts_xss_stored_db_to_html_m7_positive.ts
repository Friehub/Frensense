// [frensense]
// observation: User-supplied data from the database is rendered directly into HTML without encoding via destructured object property.
// impact: An attacker can store malicious HTML/JavaScript that executes in any viewer's browser
// improvement: Always encode user data when rendering it in HTML, or sanitize on output
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const html = `<div>${input}</div>`; res.send(html);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const html = `<article><div>${value}</div></article>`; res.send(html);
    res.json({ ok: true });
}
