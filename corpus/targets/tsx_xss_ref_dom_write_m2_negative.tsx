// SAFE: React JSX renders the intermediate variable escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const c = comment.body;
  return <div className="comment">{c}</div>;
}
