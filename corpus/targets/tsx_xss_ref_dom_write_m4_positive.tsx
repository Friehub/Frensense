// [frensense]
// observation: User content passes through a helper that does not sanitize before ref.innerHTML write.
// impact: XSS via unsanitized helper return value.
// improvement: Sanitize helper output or use React JSX.
import React, { useRef, useEffect } from "react";
function getContent(c: { body: string }): string { return c.body; }
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const content = getContent(comment);
    if (divRef.current) divRef.current.innerHTML = content;
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
