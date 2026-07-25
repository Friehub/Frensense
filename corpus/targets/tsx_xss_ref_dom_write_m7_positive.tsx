// [frensense]
// observation: User content is destructured before being written via ref.innerHTML.
// impact: XSS — destructuring does not sanitize the extracted value.
// improvement: Sanitize after destructuring or use React JSX.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
import React, { useRef, useEffect } from "react";
export function CommentRenderer({ comment }: { comment: { body: string } }) {
  const divRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const { body } = comment;
    if (divRef.current) divRef.current.innerHTML = body;
  }, [comment]);
  return <div ref={divRef} className="comment" />;
}
