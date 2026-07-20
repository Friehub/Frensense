// [frensense]
// observation: A Toast for error states sets `duration: 999999` (or `Infinity`), causing the toast to persist indefinitely and block the user's view of underlying content without any dismiss mechanism.
// impact: A persistent error toast covers part of the UI indefinitely. If the toast has no close button and the user cannot dismiss it, it creates a permanent visual obstruction. Multiple such toasts can stack and cover the entire viewport, effectively denying the user access to the application's interface.
// improvement: Use a reasonable duration (5000-8000ms) for error toasts, or provide a visible close button for persistent notifications.

import * as Toast from '@radix-ui/react-toast';
import { Button } from '@/components/ui/button';
import * as React from 'react';

export function ErrorToast() {
  const [open, setOpen] = React.useState(false);

  return (
    <Toast.Provider>
      <Button onClick={() => setOpen(true)}>Trigger Error</Button>
      <Toast.Root open={open} onOpenChange={setOpen} duration={999999}>
        <Toast.Title>Connection Lost</Toast.Title>
        <Toast.Description>Please check your internet connection and try again.</Toast.Description>
      </Toast.Root>
      <Toast.Viewport />
    </Toast.Provider>
  );
}
