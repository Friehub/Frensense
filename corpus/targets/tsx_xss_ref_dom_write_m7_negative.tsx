// SAFE: React JSX renders destructured value escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const { body } = comment;
  return <div className="comment">{body}</div>;
}
