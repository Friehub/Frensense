// [frensense]
// observation: User content is injected via template literal into ref.innerHTML.
// impact: XSS — template literal does not sanitize HTML.
// improvement: Sanitize template output or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = `${comment.body}`;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
