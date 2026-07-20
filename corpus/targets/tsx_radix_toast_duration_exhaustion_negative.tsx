// SAFE: Toast uses a reasonable duration of 8000ms and includes a Toast.Close button so the user can always dismiss it

import * as Toast from '@radix-ui/react-toast';
import { Button } from '@/components/ui/button';
import * as React from 'react';

export function ErrorToast() {
  const [open, setOpen] = React.useState(false);

  return (
    <Toast.Provider>
      <Button onClick={() => setOpen(true)}>Trigger Error</Button>
      <Toast.Root open={open} onOpenChange={setOpen} duration={8000}>
        <Toast.Title>Connection Lost</Toast.Title>
        <Toast.Description>Please check your internet connection and try again.</Toast.Description>
        <Toast.Close aria-label="Dismiss">X</Toast.Close>
      </Toast.Root>
      <Toast.Viewport />
    </Toast.Provider>
  );
}
