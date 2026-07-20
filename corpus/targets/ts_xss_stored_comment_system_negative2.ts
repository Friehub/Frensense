// SAFE: Uses a React-like server-side render with automatic escaping; comments are rendered as JSX strings (React escapes by default)
import express from "express";
import React from "react";
import { renderToString } from "react-dom/server";

function Comment({ author, body }: { author: string; body: string }) {
    return <div className="comment"><strong>{author}</strong><p>{body}</p></div>;
}

export async function renderPost(req: express.Request, res: express.Response) {
    const post = await db.query("SELECT title, author FROM posts WHERE id = ?", [req.params.id]);
    const comments = await db.query("SELECT author, body FROM comments WHERE post_id = ?", [req.params.id]);
    const html = renderToString(
        <article>
            <h1>{post[0].title}</h1>
            <p>By {post[0].author}</p>
            <section>{comments.map(c => <Comment author={c.author} body={c.body} />)}</section>
        </article>
    );
    res.send(html);
}
