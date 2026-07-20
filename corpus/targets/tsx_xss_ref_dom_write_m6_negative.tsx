// SAFE: React JSX renders concatenated value escaped
import React from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  return <div className="comment">{"<p>" + comment.body + "</p>"}</div>;
}
