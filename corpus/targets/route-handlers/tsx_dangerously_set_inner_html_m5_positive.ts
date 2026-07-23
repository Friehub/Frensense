// [frensense]
// observation: React components pass untrusted props directly into dangerouslySetInnerHTML via a template literal interpolation.
// impact: Execution of arbitrary JavaScript in the victim's browser (XSS) via malicious HTML payloads
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering

async function handlerA(req: Request, res: Response) {
    return <div dangerouslySetInnerHTML={{ __html: props.bioHtml }} />;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    return <span dangerouslySetInnerHTML={{ __html: message.content }} />;
    res.json({ ok: true });
}
