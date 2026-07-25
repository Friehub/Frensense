// [frensense]
// observation: React components pass untrusted props directly into dangerouslySetInnerHTML via string concatenation.
// impact: Execution of arbitrary JavaScript in the victim's browser (XSS) via malicious HTML payloads
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium

async function handlerA(req: Request, res: Response) {
    const html = props.bioHtml; return <div dangerouslySetInnerHTML={{ __html: html }} />;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const html = message.content; return <span dangerouslySetInnerHTML={{ __html: html }} />;
    res.json({ ok: true });
}
