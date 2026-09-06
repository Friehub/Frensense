// [frensense]
// observation: A ref is used to set innerHTML directly from user-controlled state, bypassing React's XSS protection.
// impact: User-controlled content is injected as raw HTML, enabling cross-site scripting (XSS) attacks. An attacker can execute arbitrary JavaScript in the context of the victim's browser session.
// improvement: Use textContent instead of innerHTML, or sanitize the input with DOMPurify before setting innerHTML.

import { useEffect, useRef, useState } from 'react';

export function CommentFeed() {
  const [comments, setComments] = useState<Array<{ id: string; body: string }>>([]);
  const feedRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (feedRef.current) {
      feedRef.current.innerHTML = comments
        .map(c => `<div class="comment">${c.body}</div>`)
        .join('');
    }
  }, [comments]);

  return <div ref={feedRef} />;
}
