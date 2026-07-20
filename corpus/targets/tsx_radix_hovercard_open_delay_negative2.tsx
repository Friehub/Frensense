// SAFE: Uses openDelay={800} and closeDelay={200}, and the hover card content is positioned so it never overlaps interactive trigger elements

import * as HoverCard from '@radix-ui/react-hover-card';
import { Button } from '@/components/ui/button';

export function ProfilePreview({ username }: { username: string }) {
  return (
    <HoverCard.Root openDelay={800} closeDelay={200}>
      <HoverCard.Trigger asChild>
        <span style={{ textDecoration: 'underline dotted', cursor: 'pointer' }}>{username}</span>
      </HoverCard.Trigger>
      <HoverCard.Portal>
        <HoverCard.Content side="right" sideOffset={12} align="start">
          <div style={{ padding: 12, background: '#fff', border: '1px solid #ddd', borderRadius: 8, maxWidth: 220 }}>
            <p style={{ fontWeight: 'bold' }}>{username}</p>
            <p style={{ fontSize: 12, color: '#666' }}>Joined Jan 2024</p>
          </div>
          <HoverCard.Arrow />
        </HoverCard.Content>
      </HoverCard.Portal>
    </HoverCard.Root>
  );
}
