// [frensense]
// observation: React components pass untrusted props directly into dangerouslySetInnerHTML through an intermediate variable.
// impact: Execution of arbitrary JavaScript in the victim's browser (XSS) via malicious HTML payloads
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering

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
