// SAFE: The drawer has a close button, Escape works (onEscapeKeyDown is not overridden), and pointer-down-outside is not prevented

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';

export function DismissableDrawer() {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button>Open Drawer</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content
          style={{ position: 'fixed', right: 0, top: 0, bottom: 0, width: 360, background: '#fff', padding: 24 }}
        >
          <Dialog.Title>Notifications</Dialog.Title>
          <Dialog.Description>Your recent notifications</Dialog.Description>
          <div>
            <p>No new notifications.</p>
          </div>
          <div style={{ marginTop: 24 }}>
            <Dialog.Close asChild>
              <Button variant="outline">Close Drawer</Button>
            </Dialog.Close>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
