// [frensense]
// observation: A Radix UI Popover renders its portal content without collision boundary detection, so it can render partially or fully outside the viewport bounds, allowing clickjacking of UI elements beneath the popover's visible area.
// impact: The popover can extend beyond the screen edge or render in unexpected positions, and click events can "pass through" invisible parts to underlying interactive elements. Users may accidentally trigger actions behind the popover, including destructive operations or navigation.
// improvement: Use Radix's `sideOffset` and `collisionBoundary` props, or wrap the popover content in a container with `overflow: hidden` and proper boundary detection.

import * as Popover from '@radix-ui/react-popover';
import { Button } from '@/components/ui/button';

export function ColorPickerPopover() {
  return (
    <Popover.Root>
      <Popover.Trigger asChild>
        <Button>Pick Color</Button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content sideOffset={-200} align="start" style={{ minWidth: '400px', minHeight: '600px' }}>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '4px', width: '400px' }}>
            {['red', 'blue', 'green', 'yellow', 'purple', 'orange', 'pink', 'cyan'].map((color) => (
              <div key={color} style={{ width: '80px', height: '80px', background: color, cursor: 'pointer' }} />
            ))}
          </div>
          <Popover.Arrow />
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  );
}
