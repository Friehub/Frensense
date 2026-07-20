// SAFE: Uses a template engine with auto-escaping
async function handlerA(req: Request, res: Response) {
    const comments = await db.query("SELECT body FROM comments WHERE id = ?", [req.params.id]);
    res.render("item", { body: comments[0].body });
}
async function handlerB(req: Request, res: Response) {
    const article = await db.query("SELECT content FROM articles WHERE id = ?", [req.params.id]);
    res.render("article", { content: article[0].content });
}
