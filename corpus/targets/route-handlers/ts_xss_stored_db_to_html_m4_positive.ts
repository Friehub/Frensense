// [frensense]
// observation: User-supplied data from the database is rendered directly into HTML without encoding through a helper function.
// impact: An attacker can store malicious HTML/JavaScript that executes in any viewer's browser
// improvement: Always encode user data when rendering it in HTML, or sanitize on output
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(comment.body);
    const html = `<div>${val}</div>`; res.send(html);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(article.content);
    const html = `<article><div>${val}</div></article>`; res.send(html);
    res.json({ ok: true });
}
