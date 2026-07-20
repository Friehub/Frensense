// SAFE: React JSX renders array element escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string[] } }) {
  return <div className="comment">{comment.body[0]}</div>;
}
