// [frensense]
// observation: User-supplied data from the database is rendered directly into HTML without encoding through multiple variable assignments.
// impact: An attacker can store malicious HTML/JavaScript that executes in any viewer's browser
// improvement: Always encode user data when rendering it in HTML, or sanitize on output
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

async function handlerA(req: Request, res: Response) {
    const a = comment.body;
    const b = a;
    const html = `<div>${b}</div>`; res.send(html);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = article.content;
    const y = x;
    const z = y;
    const html = `<article><div>${z}</div></article>`; res.send(html);
    res.json({ ok: true });
}
