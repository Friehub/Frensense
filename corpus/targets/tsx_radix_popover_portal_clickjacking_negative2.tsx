// SAFE: Popover content is wrapped in a bounded container with overflow hidden, and uses Radix's built-in collision avoidance

import * as Popover from '@radix-ui/react-popover';
import { Button } from '@/components/ui/button';

export function ColorPickerPopover() {
  return (
    <Popover.Root>
      <Popover.Trigger asChild>
        <Button>Pick Color</Button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content
          sideOffset={4}
          avoidCollisions
          collisionPadding={16}
          style={{ width: '280px' }}
        >
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '4px' }}>
            {['red', 'blue', 'green', 'yellow', 'purple', 'orange', 'pink', 'cyan'].map((color) => (
              <div key={color} style={{ width: '60px', height: '60px', background: color, cursor: 'pointer' }} />
            ))}
          </div>
          <Popover.Arrow />
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  );
}
