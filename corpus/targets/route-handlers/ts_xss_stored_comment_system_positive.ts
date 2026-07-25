// [frensense]
// observation: Comments, titles, and author names from users are rendered on a page without sanitization, enabling stored XSS across all visitors.
// impact: An attacker posts a comment containing <script> tags that steal session cookies or perform actions on behalf of other users viewing the comment thread.
// improvement: Sanitize all user-submitted content when rendering, or use HTML-encoding for comment body, title, and author fields.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export async function renderPost(req: express.Request, res: express.Response) {
    const post = await db.query("SELECT title, author FROM posts WHERE id = ?", [req.params.id]);
    const comments = await db.query("SELECT author, body FROM comments WHERE post_id = ?", [req.params.id]);
    let html = `<article><h1>${post[0].title}</h1><p>By ${post[0].author}</p></article><section>`;
    for (const c of comments) {
        html += `<div class="comment"><strong>${c.author}</strong><p>${c.body}</p></div>`;
    }
    html += "</section>";
    res.send(html);
}
