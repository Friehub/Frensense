// [frensense]
// observation: User-supplied data from the database is rendered directly into HTML without encoding via string concatenation.
// impact: An attacker can store malicious HTML/JavaScript that executes in any viewer's browser
// improvement: Always encode user data when rendering it in HTML, or sanitize on output

async function handlerA(req: Request, res: Response) {
    const html = "<div>" + comment.body + "</div>"; res.send(html);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const html = "<article><div>" + article.content + "</div></article>"; res.send(html);
    res.json({ ok: true });
}
