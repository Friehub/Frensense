// [frensense]
// observation: User content is concatenated before being written via ref.innerHTML.
// impact: XSS — concatenation does not sanitize embedded HTML.
// improvement: Sanitize the concatenated result or use React JSX.
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (divRef.current) divRef.current.innerHTML = "<p>" + comment.body + "</p>";
  }, [comment.body]);
  return <div ref={divRef} className="comment" />;
}
