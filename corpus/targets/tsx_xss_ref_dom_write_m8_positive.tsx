// [frensense]
// observation: User content is accessed via array index before ref.innerHTML write.
// impact: XSS — array element is not sanitized.
// improvement: Sanitize the array element or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string[] } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = comment.body[0];
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
