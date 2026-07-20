// SAFE: onEscapeKeyDown only blocks close when there are unsaved changes, and provides an alternative dismiss path via a Cancel button

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';

interface UnsavedChangesDialogProps {
  hasUnsavedChanges: boolean;
}

export function UnsavedChangesDialog({ hasUnsavedChanges }: UnsavedChangesDialogProps) {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <Button>Edit Profile</Button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content
          onEscapeKeyDown={(e) => {
            if (hasUnsavedChanges) e.preventDefault();
          }}
        >
          <Dialog.Title>Edit Profile</Dialog.Title>
          <Dialog.Description>Make your changes below.</Dialog.Description>
          {hasUnsavedChanges ? (
            <p style={{ color: 'red' }}>You have unsaved changes. Use Cancel to dismiss.</p>
          ) : null}
          <div>
            <Dialog.Close asChild>
              <Button variant="outline">Cancel</Button>
            </Dialog.Close>
            <Button type="submit">Save</Button>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
