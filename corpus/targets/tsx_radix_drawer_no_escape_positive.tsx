// [frensense]
// observation: A Drawer is opened without any close mechanism — no close button, no Escape key handler, and clicking outside does not dismiss it, trapping the user in the open drawer state.
// impact: Once the drawer opens, the user cannot dismiss it. The drawer covers important content and the user is forced to refresh the page to regain access to the underlying UI. This violates WCAG 2.1.2 (No Keyboard Trap) and creates a denial-of-service condition for the user.
// improvement: Always provide a close button, enable Escape dismissal, and/or allow clicking outside to close. Never disable all dismiss mechanisms simultaneously.

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';
import { Cross2Icon } from '@radix-ui/react-icons';

export function StuckDrawer() {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button>Open Drawer</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay style={{ pointerEvents: 'none' }} />
        <Dialog.Content
          onEscapeKeyDown={(e) => e.preventDefault()}
          onPointerDownOutside={(e) => e.preventDefault()}
          style={{ position: 'fixed', right: 0, top: 0, bottom: 0, width: 400, background: '#fff' }}
        >
          <Dialog.Title>Notifications</Dialog.Title>
          <Dialog.Description>Your recent notifications</Dialog.Description>
          <div style={{ padding: 16 }}>
            <p>No new notifications.</p>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
