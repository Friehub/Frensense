// SAFE: Menu items use a type-action mapping that only allows registered callback functions, preventing arbitrary function injection

import * as DropdownMenu from '@radix-ui/react-dropdown-menu';
import { Button } from '@/components/ui/button';

type MenuAction = 'edit' | 'delete' | 'share' | 'export';

const ACTION_HANDLERS: Record<MenuAction, (id: string) => void> = {
  edit: (id) => console.log('Edit', id),
  delete: (id) => console.log('Delete', id),
  share: (id) => console.log('Share', id),
  export: (id) => console.log('Export', id),
};

interface MenuItemConfig {
  label: string;
  action: MenuAction;
}

export function DynamicMenu({ resourceId, items }: { resourceId: string; items: MenuItemConfig[] }) {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <Button>Actions</Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content>
        {items.map((item, idx) => {
          const handler = ACTION_HANDLERS[item.action];
          if (!handler) return null;
          return (
            <DropdownMenu.Item key={idx} onSelect={() => handler(resourceId)}>
              {item.label}
            </DropdownMenu.Item>
          );
        })}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  );
}
