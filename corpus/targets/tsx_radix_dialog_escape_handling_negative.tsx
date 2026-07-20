// SAFE: The dialog does not override onEscapeKeyDown, so Escape closes naturally without trapping keyboard users

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';

export function ConfirmDeleteDialog() {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button variant="destructive">Delete Item</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content>
          <Dialog.Title>Confirm Deletion</Dialog.Title>
          <Dialog.Description>This action cannot be undone.</Dialog.Description>
          <Dialog.Close asChild>
            <Button variant="destructive">Confirm</Button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
