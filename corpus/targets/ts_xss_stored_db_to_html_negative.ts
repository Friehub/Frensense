// SAFE: All dynamic content from the database is HTML-escaped before rendering
import express from "express";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export async function getComments(req: express.Request, res: express.Response) {
    const comments = await db.query("SELECT body, author FROM comments WHERE post_id = ?", [req.params.postId]);
    let html = "<ul>";
    for (const c of comments) {
        html += `<li><strong>${escapeHtml(c.author)}</strong>: ${escapeHtml(c.body)}</li>`;
    }
    html += "</ul>";
    res.send(html);
}

export async function getArticle(req: express.Request, res: express.Response) {
    const article = await db.query("SELECT title, content FROM articles WHERE id = ?", [req.params.id]);
    const title = escapeHtml(article[0].title);
    const content = escapeHtml(article[0].content);
    res.send(`<article><h1>${title}</h1><div>${content}</div></article>`);
}
