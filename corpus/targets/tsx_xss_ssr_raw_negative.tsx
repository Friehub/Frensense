// SAFE: HTML is sanitized on the server before being passed to the component, preventing SSR XSS
import React from "react";
import sanitizeHtml from "sanitize-html";

interface PageProps {
    content: string;
}

export function SSRPage({ content }: PageProps) {
    const safe = sanitizeHtml(content, {
        allowedTags: ["b", "i", "em", "strong", "p", "br"],
        allowedAttributes: {},
    });
    return (
        <div>
            <h1>User Content</h1>
            <div dangerouslySetInnerHTML={{ __html: safe }} />
        </div>
    );
}

export function BlogPost({ post }: { post: { title: string; bodyHtml: string } }) {
    const safe = sanitizeHtml(post.bodyHtml, {
        allowedTags: ["b", "i", "em", "strong", "p", "br", "ul", "ol", "li"],
        allowedAttributes: {},
    });
    return (
        <article>
            <h1>{post.title}</h1>
            <div className="body" dangerouslySetInnerHTML={{ __html: safe }} />
        </article>
    );
}
