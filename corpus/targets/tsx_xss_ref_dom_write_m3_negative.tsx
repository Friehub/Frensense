// SAFE: React JSX renders multi-hop value escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const a = comment.body;
  const b = a;
  return <div className="comment">{b}</div>;
}
