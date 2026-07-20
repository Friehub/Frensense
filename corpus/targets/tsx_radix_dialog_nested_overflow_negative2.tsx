// SAFE: When nested dialogs are unavoidable, explicit z-index and a global portal container ensure stacking order is correct

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';
import { useState } from 'react';

const PARENT_Z = 50;
const CHILD_Z = 100;

export function NestedDialogs() {
  const [nestedOpen, setNestedOpen] = useState(false);

  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button>Open Settings</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay style={{ zIndex: PARENT_Z }} />
        <Dialog.Content style={{ zIndex: PARENT_Z + 1 }}>
          <Dialog.Title>Settings</Dialog.Title>
          <Dialog.Description>Manage your preferences.</Dialog.Description>
          <Button onClick={() => setNestedOpen(true)}>Advanced Options</Button>
          <Dialog.Root open={nestedOpen} onOpenChange={setNestedOpen}>
            <Dialog.Portal>
              <Dialog.Overlay style={{ zIndex: CHILD_Z }} />
              <Dialog.Content style={{ zIndex: CHILD_Z + 1 }}>
                <Dialog.Title>Advanced Options</Dialog.Title>
                <Dialog.Description>Dangerous settings.</Dialog.Description>
                <Dialog.Close asChild>
                  <Button>Close Advanced</Button>
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
