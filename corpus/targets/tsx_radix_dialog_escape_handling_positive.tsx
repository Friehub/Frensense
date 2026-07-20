// [frensense]
// observation: A Dialog's `onEscapeKeyDown` handler calls `event.preventDefault()` unconditionally, preventing the dialog from closing when the user presses Escape.
// impact: Users cannot dismiss the dialog via keyboard escape, trapping keyboard-only users in a modal. This violates WCAG 2.1.2 (No Keyboard Trap) and may force users to refresh the page.
// improvement: Either omit `onEscapeKeyDown` entirely, or call `event.preventDefault()` only when the dialog has unsaved changes and provide an alternative dismiss path.

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
        <Dialog.Content onEscapeKeyDown={(e) => e.preventDefault()}>
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
