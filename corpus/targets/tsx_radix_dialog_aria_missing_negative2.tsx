// SAFE: aria-label is provided directly on Dialog.Content so screen readers can announce the dialog purpose even without a visible title

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
        <Dialog.Content aria-label="Confirm deletion">
          <Dialog.Description>Are you sure you want to delete this item?</Dialog.Description>
          <Dialog.Close asChild>
            <Button>Confirm</Button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
