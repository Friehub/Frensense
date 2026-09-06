// SAFE: Avoids dangerouslySetInnerHTML entirely; uses React's JSX escaping for rendering
import React from "react";
import { marked } from "marked";

interface PageProps {
    content: string;
}

export function SSRPage({ content }: PageProps) {
    return (
        <div>
            <h1>User Content</h1>
            <div>{content}</div>
        </div>
    );
}

export function BlogPost({ post }: { post: { title: string; bodyHtml: string } }) {
    return (
        <article>
            <h1>{post.title}</h1>
            <div className="body">{post.bodyHtml}</div>
        </article>
    );
}
