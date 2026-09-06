// [frensense]
// observation: A React component uses a ref to access a DOM element and sets its innerHTML directly with user-controlled data, bypassing React's escaping.
// impact: User input flows through the ref to innerHTML, enabling XSS even in a React app because refs bypass React's protection mechanisms.
// improvement: Use React state and JSX to render dynamic content, or sanitize the input before assigning to innerHTML via ref.

import React, { useRef, useEffect } from "react";

export function CommentRenderer({ comment }: { comment: { body: string } }) {
    const divRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (divRef.current) {
            divRef.current.innerHTML = comment.body;
        }
    }, [comment.body]);

    return <div ref={divRef} className="comment" />;
}

export function RichTextDisplay({ html }: { html: string }) {
    const containerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (containerRef.current) {
            containerRef.current.innerHTML = html;
        }
    }, [html]);

    return <div ref={containerRef} className="rich-text" />;
}
