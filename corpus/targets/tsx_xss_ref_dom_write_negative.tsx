// SAFE: Uses React JSX to render content instead of ref-based innerHTML; React escapes the output
import React from "react";

export function CommentRenderer({ comment }: { comment: { body: string } }) {
    return <div className="comment">{comment.body}</div>;
}

export function RichTextDisplay({ html }: { html: string }) {
    return <div className="rich-text">{html}</div>;
}
