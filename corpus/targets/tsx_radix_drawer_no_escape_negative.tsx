// SAFE: A close button is provided, Escape key is not intercepted, and the overlay can be clicked to dismiss

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';
import { Cross2Icon } from '@radix-ui/react-icons';

export function DismissableDrawer() {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button>Open Drawer</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content
          style={{ position: 'fixed', right: 0, top: 0, bottom: 0, width: 400, background: '#fff' }}
        >
          <Dialog.Title>Notifications</Dialog.Title>
          <Dialog.Description>Your recent notifications</Dialog.Description>
          <div style={{ padding: 16 }}>
            <p>No new notifications.</p>
          </div>
          <Dialog.Close asChild>
            <button aria-label="Close" style={{ position: 'absolute', top: 8, right: 8 }}>
              <Cross2Icon />
            </button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
