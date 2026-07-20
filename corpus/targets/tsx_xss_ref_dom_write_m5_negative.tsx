// SAFE: React JSX renders template literal escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  return <div className="comment">{`${comment.body}`}</div>;
}
