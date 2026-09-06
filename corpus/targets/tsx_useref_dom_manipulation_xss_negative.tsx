// SAFE: uses textContent instead of innerHTML, preventing HTML injection

import { useEffect, useRef, useState } from 'react';

export function CommentFeed() {
  const [comments, setComments] = useState<Array<{ id: string; body: string }>>([]);
  const feedRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (feedRef.current) {
      feedRef.current.textContent = comments
        .map(c => c.body)
        .join('\n');
    }
  }, [comments]);

  return <div ref={feedRef} />;
}
