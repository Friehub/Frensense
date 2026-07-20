// SAFE: destructured value is sanitized before innerHTML write
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const { body } = comment;
    if (divRef.current) divRef.current.innerHTML = DOMPurify.sanitize(body);
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
