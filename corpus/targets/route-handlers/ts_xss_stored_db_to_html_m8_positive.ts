// [frensense]
// observation: User-supplied data from the database is rendered directly into HTML without encoding via an array element access.
// impact: An attacker can store malicious HTML/JavaScript that executes in any viewer's browser
// improvement: Always encode user data when rendering it in HTML, or sanitize on output
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

async function handlerA(req: Request, res: Response) {
    const arr = [comment.body];
    const html = `<div>${arr[0]}</div>`; res.send(html);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [article.content];
    const html = `<article><div>${items[0]}</div></article>`; res.send(html);
    res.json({ ok: true });
}
