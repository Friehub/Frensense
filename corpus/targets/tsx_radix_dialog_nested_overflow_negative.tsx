// SAFE: Avoids nested dialogs entirely — advanced options appear inline rather than stacking another modal on top

import * as Dialog from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';
import { useState } from 'react';

export function SettingsDialog() {
  const [showAdvanced, setShowAdvanced] = useState(false);

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
          {showAdvanced ? (
            <div style={{ borderTop: '1px solid #ccc', marginTop: 8, paddingTop: 8 }}>
              <p style={{ fontWeight: 'bold' }}>Advanced Options</p>
              <p>Dangerous settings appear inline here.</p>
              <Button onClick={() => setShowAdvanced(false)}>Hide Advanced</Button>
            </div>
          ) : (
            <Button onClick={() => setShowAdvanced(true)}>Show Advanced Options</Button>
          )}
          <Dialog.Close asChild>
            <Button variant="outline">Close</Button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
