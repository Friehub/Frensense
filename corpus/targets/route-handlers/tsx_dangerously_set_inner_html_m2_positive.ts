// [frensense]
// observation: React components pass untrusted props directly into dangerouslySetInnerHTML through an intermediate variable.
// impact: Execution of arbitrary JavaScript in the victim's browser (XSS) via malicious HTML payloads
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium

async function handlerA(req: Request, res: Response) {
    const val = props.bioHtml;
    return <div dangerouslySetInnerHTML={{ __html: val }} />;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = message.content;
    return <span dangerouslySetInnerHTML={{ __html: val }} />;
    res.json({ ok: true });
}
