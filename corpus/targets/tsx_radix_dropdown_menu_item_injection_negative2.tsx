// SAFE: Menu items are built from a hardcoded configuration with no dynamic onSelect — all handlers are explicitly defined in code

import * as DropdownMenu from '@radix-ui/react-dropdown-menu';
import { Button } from '@/components/ui/button';

interface DynamicMenuProps {
  resourceId: string;
  onEdit: (id: string) => void;
  onDelete: (id: string) => void;
}

const MENU_ITEMS = [
  { label: 'Edit', id: 'edit' as const },
  { label: 'Delete', id: 'delete' as const },
  { label: 'Share', id: 'share' as const },
];

export function DynamicMenu({ resourceId, onEdit, onDelete }: DynamicMenuProps) {
  const handleSelect = (id: string) => {
    switch (id) {
      case 'edit': onEdit(resourceId); break;
      case 'delete': onDelete(resourceId); break;
      case 'share': navigator.clipboard.writeText(window.location.href); break;
    }
  };

  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <Button>Actions</Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content>
        {MENU_ITEMS.map((item) => (
          <DropdownMenu.Item key={item.id} onSelect={() => handleSelect(item.id)}>
            {item.label}
          </DropdownMenu.Item>
        ))}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  );
}
