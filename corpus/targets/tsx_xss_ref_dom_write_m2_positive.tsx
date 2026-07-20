// [frensense]
// observation: User content is assigned to an intermediate variable before being written via ref.innerHTML.
// impact: XSS — intermediate variable carries unsanitized HTML to innerHTML.
// improvement: Sanitize or use React JSX instead of ref innerHTML.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const c = comment.body;
    if (divRef.current) divRef.current.innerHTML = c;
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
