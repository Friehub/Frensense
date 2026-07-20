// SAFE: if HTML is required, DOMPurify is used to sanitize user content before injection

import { useEffect, useRef, useState } from 'react';
import DOMPurify from 'dompurify';

export function CommentFeed() {
  const [comments, setComments] = useState<Array<{ id: string; body: string }>>([]);
  const feedRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (feedRef.current) {
      feedRef.current.innerHTML = comments
        .map(c => `<div class="comment">${DOMPurify.sanitize(c.body)}</div>`)
        .join('');
    }
  }, [comments]);

  return <div ref={feedRef} />;
}
