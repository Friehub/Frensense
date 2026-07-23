// [frensense]
// observation: React components pass untrusted props directly into dangerouslySetInnerHTML via destructured object property.
// impact: Execution of arbitrary JavaScript in the victim's browser (XSS) via malicious HTML payloads
// improvement: Sanitize the HTML using DOMPurify before injecting it, or prefer standard React rendering

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const { input } = props; return <div dangerouslySetInnerHTML={{ __html: input }} />;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const { value } = message; return <span dangerouslySetInnerHTML={{ __html: value }} />;
    res.json({ ok: true });
}
