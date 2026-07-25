// [frensense]
// observation: A server-rendered React component passes user-controlled data via dangerouslySetInnerHTML through SSR props, allowing XSS before any client sanitization runs.
// impact: User input rendered server-side in __html content executes in the browser. SSR XSS can bypass client-side sanitization if the sanitization only runs on the client.
// improvement: Sanitize HTML on the server before SSR, or avoid dangerouslySetInnerHTML with user-controlled data entirely.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import React from "react";

interface PageProps {
    content: string;
}

export function SSRPage({ content }: PageProps) {
    return (
        <div>
            <h1>User Content</h1>
            <div dangerouslySetInnerHTML={{ __html: content }} />
        </div>
    );
}

export function BlogPost({ post }: { post: { title: string; bodyHtml: string } }) {
    return (
        <article>
            <h1>{post.title}</h1>
            <div className="body" dangerouslySetInnerHTML={{ __html: post.bodyHtml }} />
        </article>
    );
}
