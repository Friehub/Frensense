// [frensense]
// observation: A Popover is configured with `side: 'bottom'` and a large `sideOffset` but no `collisionBoundary` or `collisionPadding`, causing it to render outside the viewport and flicker as Radix's collision detection continuously flips the side.
// impact: The popover visibly flickers between bottom and top positions as the browser re-renders, creating a poor UX and potentially causing misclicks if the user attempts to interact during the flicker. The flicker can also trigger unwanted scroll or resize events.
// improvement: Always set `collisionBoundary` and `collisionPadding` when using a fixed `side`, or omit `side` to let Radix auto-position.

import * as Popover from '@radix-ui/react-popover';
import { Button } from '@/components/ui/button';

export function BottomPopover() {
  return (
    <Popover.Root>
      <Popover.Trigger asChild>
        <Button>Options</Button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content side="bottom" sideOffset={40} align="center">
          <div style={{ padding: 16, background: '#fff', border: '1px solid #ccc', borderRadius: 8 }}>
            <p>Menu item 1</p>
            <p>Menu item 2</p>
            <p>Menu item 3</p>
          </div>
          <Popover.Arrow />
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  );
}
