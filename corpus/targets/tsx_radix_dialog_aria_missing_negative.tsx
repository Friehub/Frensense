// SAFE: Dialog.Title is present and Dialog.Content uses aria-labelledby referencing the title id, giving screen readers proper dialog context

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';

export function DeleteDialog() {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button variant="destructive">Delete</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content aria-labelledby="dialog-title">
          <Dialog.Title id="dialog-title">Confirm Deletion</Dialog.Title>
          <Dialog.Description>Are you sure you want to delete this item?</Dialog.Description>
          <Dialog.Close asChild>
            <Button>Confirm</Button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
