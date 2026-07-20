// SAFE: helper sanitizes before returning
import React, { useRef, useEffect } from "react";
import DOMPurify from "dompurify";
function getSafeContent(c: { body: string }): string { return DOMPurify.sanitize(c.body); }
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const content = getSafeContent(comment);
    if (divRef.current) divRef.current.innerHTML = content;
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
