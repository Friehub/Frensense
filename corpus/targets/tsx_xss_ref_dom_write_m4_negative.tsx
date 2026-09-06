// SAFE: React JSX renders helper output escaped
import React from "react";
function getContent(c: { body: string }): string { return c.body; }
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const content = getContent(comment);
  return <div className="comment">{content}</div>;
}
