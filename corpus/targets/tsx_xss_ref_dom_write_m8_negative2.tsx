// SAFE: array element is sanitized before innerHTML write
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string[] } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = DOMPurify.sanitize(comment.body[0]);
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
