// SAFE: Uses a template engine with auto-escaping to render database content
import express from "express";

export async function getComments(req: express.Request, res: express.Response) {
    const comments = await db.query("SELECT body, author FROM comments WHERE post_id = ?", [req.params.postId]);
    res.render("comments", { comments });
}

export async function getArticle(req: express.Request, res: express.Response) {
    const article = await db.query("SELECT title, content FROM articles WHERE id = ?", [req.params.id]);
    res.render("article", { title: article[0].title, content: article[0].content });
}
