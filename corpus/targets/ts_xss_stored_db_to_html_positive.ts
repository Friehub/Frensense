// [frensense]
// observation: User-supplied data is stored in a database and later rendered directly into an HTML page without encoding or sanitization.
// impact: An attacker can store malicious HTML/JavaScript that executes in the browser of any user viewing the page, leading to persistent XSS.
// improvement: Always encode user data when rendering it in HTML, or sanitize on output.

import express from "express";

export async function getComments(req: express.Request, res: express.Response) {
    const comments = await db.query("SELECT body, author FROM comments WHERE post_id = ?", [req.params.postId]);
    let html = "<ul>";
    for (const c of comments) {
        html += `<li><strong>${c.author}</strong>: ${c.body}</li>`;
    }
    html += "</ul>";
    res.send(html);
}

export async function getArticle(req: express.Request, res: express.Response) {
    const article = await db.query("SELECT title, content FROM articles WHERE id = ?", [req.params.id]);
    res.send(`<article><h1>${article[0].title}</h1><div>${article[0].content}</div></article>`);
}
