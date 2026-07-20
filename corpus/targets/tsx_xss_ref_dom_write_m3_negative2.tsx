// SAFE: sanitization applied at final hop
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const a = comment.body;
    const b = DOMPurify.sanitize(a);
    if (divRef.current) divRef.current.innerHTML = b;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
