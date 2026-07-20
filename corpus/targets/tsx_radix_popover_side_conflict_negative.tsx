// SAFE: side is omitted so Radix auto-positions, and collisionBoundary with padding ensures the popover stays within the viewport

import * as Popover from '@radix-ui/react-popover';
import { Button } from '@/components/ui/button';

export function BottomPopover() {
  return (
    <Popover.Root>
      <Popover.Trigger asChild>
        <Button>Options</Button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content collisionBoundary={document.body} collisionPadding={16} align="center">
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
