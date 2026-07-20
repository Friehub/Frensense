// [frensense]
// observation: A HoverCard uses `openDelay: 0` and `closeDelay: 100`, causing the card to appear instantly on any mouse movement over the trigger, leading to accidental triggers and potential UI redressing.
// impact: The hover card appears on even the slightest mouse movement, covering underlying content. An attacker could position a transparent hover card over sensitive UI elements (e.g., a "Delete" button) so that when the user unintentionally triggers it, the layout shifts and the user clicks a destructive action (UI redressing / clickjacking).
// improvement: Use a reasonable `openDelay` (300ms or more) and ensure the hover card does not cover interactive elements.

import * as HoverCard from '@radix-ui/react-hover-card';
import { Button } from '@/components/ui/button';

export function InstantPreview({ userId }: { userId: string }) {
  return (
    <HoverCard.Root openDelay={0} closeDelay={100}>
      <HoverCard.Trigger asChild>
        <Button>View Profile</Button>
      </HoverCard.Trigger>
      <HoverCard.Portal>
        <HoverCard.Content sideOffset={4}>
          <div style={{ padding: 16, background: '#fff', border: '1px solid #ccc', borderRadius: 8, width: 300 }}>
            <p style={{ fontWeight: 'bold' }}>User {userId}</p>
            <p>Bio and details appear instantly on hover.</p>
          </div>
          <HoverCard.Arrow />
        </HoverCard.Content>
      </HoverCard.Portal>
    </HoverCard.Root>
  );
}
