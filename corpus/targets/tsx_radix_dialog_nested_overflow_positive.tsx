// [frensense]
// observation: A Dialog opens another Dialog on top without specifying explicit z-index values, causing the nested dialog to render behind or at the same layer as the overlay of the first dialog, making the nested dialog unreachable.
// impact: When the second dialog opens, it renders behind the parent dialog's overlay, making it invisible and unclickable. The application appears frozen and the user cannot interact with the nested dialog or dismiss it, effectively trapping the user.
// improvement: Manage z-index values explicitly for each modal layer, or avoid nested dialogs by using inline confirmations or stacked notifications.

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';
import { useState } from 'react';

export function NestedDialogs() {
  const [nestedOpen, setNestedOpen] = useState(false);

  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button>Open Settings</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content>
          <Dialog.Title>Settings</Dialog.Title>
          <Dialog.Description>Manage your preferences.</Dialog.Description>
          <Button onClick={() => setNestedOpen(true)}>Advanced Options</Button>
          <Dialog.Root open={nestedOpen} onOpenChange={setNestedOpen}>
            <Dialog.Portal>
              <Dialog.Overlay />
              <Dialog.Content>
                <Dialog.Title>Advanced Options</Dialog.Title>
                <Dialog.Description>Dangerous settings.</Dialog.Description>
                <Dialog.Close asChild>
                  <Button>Close</Button>
                </Dialog.Close>
              </Dialog.Content>
            </Dialog.Portal>
          </Dialog.Root>
          <Dialog.Close asChild>
            <Button variant="outline">Close</Button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
