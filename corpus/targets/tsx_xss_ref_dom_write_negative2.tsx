// SAFE: Assigns to innerHTML only after sanitizing with DOMPurify
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";

export function CommentRenderer({ comment }: { comment: { body: string } }) {
    const divRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (divRef.current) {
            divRef.current.innerHTML = DOMPurify.sanitize(comment.body);
        }
    }, [comment.body]);

    return <div ref={divRef} className="comment" />;
}

export function RichTextDisplay({ html }: { html: string }) {
    const containerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (containerRef.current) {
            containerRef.current.innerHTML = DOMPurify.sanitize(html);
        }
    }, [html]);

    return <div ref={containerRef} className="rich-text" />;
}
