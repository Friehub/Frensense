// [frensense]
// observation: A Dialog is rendered without `aria-labelledby` or `aria-label`, and without a visible `Dialog.Title`, so screen readers cannot identify the dialog's purpose to users.
// impact: Assistive technology users hear an unlabeled "dialog" announcement with no context about its purpose, making the dialog inaccessible and failing WCAG 4.1.2 (Name, Role, Value).
// improvement: Always include a `Dialog.Title` and ensure it is referenced via `aria-labelledby` on `Dialog.Content`.

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
        <Dialog.Content>
          <p>Are you sure you want to delete this item?</p>
          <Dialog.Close asChild>
            <Button>Confirm</Button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
