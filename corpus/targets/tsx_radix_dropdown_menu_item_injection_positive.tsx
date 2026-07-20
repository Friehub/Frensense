// [frensense]
// observation: A Radix UI Dropdown Menu item's `onSelect` handler is set dynamically from user-controlled content (e.g., menu item data fetched from an API or derived from URL params), allowing an attacker to inject an arbitrary callback.
// impact: An attacker who controls menu item metadata can set onSelect to any function reference in scope, including `eval`, `document.write`, or sensitive internal functions, leading to arbitrary code execution within the application context.
// improvement: Use a lookup table mapping item IDs to permitted callback functions, never pass user-controlled functions as event handlers.

import * as DropdownMenu from '@radix-ui/react-dropdown-menu';
import { Button } from '@/components/ui/button';

interface MenuItem {
  label: string;
  onSelect: () => void;
}

export function DynamicMenu({ items }: { items: MenuItem[] }) {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <Button>Actions</Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content>
        {items.map((item, idx) => (
          <DropdownMenu.Item key={idx} onSelect={item.onSelect}>
            {item.label}
          </DropdownMenu.Item>
        ))}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  );
}
