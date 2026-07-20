// SAFE: Uses swipe-to-dismiss and a close button, and the duration is capped at 10000ms so the toast cannot block indefinitely

import * as Toast from '@radix-ui/react-toast';
import { Button } from '@/components/ui/button';
import * as React from 'react';

const MAX_TOAST_DURATION = 10_000;

export function ErrorToast() {
  const [open, setOpen] = React.useState(false);

  return (
    <Toast.Provider swipeDirection="right">
      <Button onClick={() => setOpen(true)}>Trigger Error</Button>
      <Toast.Root open={open} onOpenChange={setOpen} duration={MAX_TOAST_DURATION}>
        <Toast.Title>Connection Lost</Toast.Title>
        <Toast.Description>Please check your internet connection.</Toast.Description>
        <Toast.Action asChild altText="Retry connection">
          <Button variant="outline" size="sm" onClick={() => window.location.reload()}>Retry</Button>
        </Toast.Action>
        <Toast.Close aria-label="Dismiss" />
      </Toast.Root>
      <Toast.Viewport />
    </Toast.Provider>
  );
}
