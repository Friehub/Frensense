// [frensense]
// observation: React components pass untrusted props directly into dangerouslySetInnerHTML via an array element access.
// impact: Execution of arbitrary JavaScript in the victim's browser (XSS) via malicious HTML payloads
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering

async function handlerA(req: Request, res: Response) {
    const arr = [props.bioHtml];
    return <div dangerouslySetInnerHTML={{ __html: arr[0] }} />;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [message.content];
    return <span dangerouslySetInnerHTML={{ __html: items[0] }} />;
    res.json({ ok: true });
}
