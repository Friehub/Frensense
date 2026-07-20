// SAFE: openDelay is set to 700ms so the hover card only appears after a deliberate hover, preventing accidental triggers

import * as HoverCard from '@radix-ui/react-hover-card';
import { Button } from '@/components/ui/button';

export function DelayedPreview({ userId }: { userId: string }) {
  return (
    <HoverCard.Root openDelay={700} closeDelay={300}>
      <HoverCard.Trigger asChild>
        <Button>View Profile</Button>
      </HoverCard.Trigger>
      <HoverCard.Portal>
        <HoverCard.Content sideOffset={4}>
          <div style={{ padding: 16, background: '#fff', border: '1px solid #ccc', borderRadius: 8, width: 300 }}>
            <p style={{ fontWeight: 'bold' }}>User {userId}</p>
            <p>Bio and details appear after a deliberate hover.</p>
          </div>
          <HoverCard.Arrow />
        </HoverCard.Content>
      </HoverCard.Portal>
    </HoverCard.Root>
  );
}
