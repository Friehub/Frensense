// SAFE: Implements safe alternative
// SAFE: All dynamic content is HTML-escaped before rendering
function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}
async function handlerA(req: Request, res: Response) {
    const comment = await db.query("SELECT body FROM comments WHERE id = ?", [req.params.id]);
    res.send(`<div>${escapeHtml(comment[0].body)}</div>`);
}
async function handlerB(req: Request, res: Response) {
    const article = await db.query("SELECT content FROM articles WHERE id = ?", [req.params.id]);
    res.send(`<article><div>${escapeHtml(article[0].content)}</div></article>`);
}
