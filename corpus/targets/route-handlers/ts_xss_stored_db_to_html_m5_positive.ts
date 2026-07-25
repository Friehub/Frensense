// [frensense]
// observation: User-supplied data from the database is rendered directly into HTML without encoding via a template literal interpolation.
// impact: An attacker can store malicious HTML/JavaScript that executes in any viewer's browser
// improvement: Always encode user data when rendering it in HTML, or sanitize on output
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

async function handlerA(req: Request, res: Response) {
    res.send(`<div>${comment.body}</div>`);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    res.send(`<article><div>${article.content}</div></article>`);
    res.json({ ok: true });
}
