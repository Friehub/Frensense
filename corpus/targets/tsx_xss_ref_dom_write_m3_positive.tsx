// [frensense]
// observation: User content flows through two assignments before ref.innerHTML write.
// impact: XSS via multi-hop DOM write.
// improvement: Sanitize at any hop or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const a = comment.body;
    const b = a;
    if (divRef.current) divRef.current.innerHTML = b;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
