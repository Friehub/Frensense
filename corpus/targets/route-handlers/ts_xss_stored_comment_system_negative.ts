// SAFE: Comments are HTML-escaped before rendering
import express from "express";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export async function renderPost(req: express.Request, res: express.Response) {
    const post = await db.query("SELECT title, author FROM posts WHERE id = ?", [req.params.id]);
    const comments = await db.query("SELECT author, body FROM comments WHERE post_id = ?", [req.params.id]);
    let html = `<article><h1>${escapeHtml(post[0].title)}</h1><p>By ${escapeHtml(post[0].author)}</p></article><section>`;
    for (const c of comments) {
        html += `<div class="comment"><strong>${escapeHtml(c.author)}</strong><p>${escapeHtml(c.body)}</p></div>`;
    }
    html += "</section>";
    res.send(html);
}
