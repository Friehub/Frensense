// SAFE: intermediate variable is sanitized via DOMPurify before innerHTML assignment
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const c = DOMPurify.sanitize(comment.body);
    if (divRef.current) divRef.current.innerHTML = c;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
