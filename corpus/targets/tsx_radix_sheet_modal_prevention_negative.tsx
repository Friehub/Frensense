// SAFE: Modal is true (default), so background content cannot be interacted with while the sheet is open

import * as Sheet from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';

export function SettingsSheet() {
  return (
    <Sheet.Root>
      <Sheet.Trigger asChild>
        <Button>Open Settings</Button>
      </Sheet.Trigger>
      <Sheet.Portal>
        <Sheet.Overlay />
        <Sheet.Content>
          <Sheet.Title>Settings</Sheet.Title>
          <Sheet.Description>Configure your preferences</Sheet.Description>
          <div style={{ padding: 16 }}>
            <label>
              <input type="checkbox" /> Enable notifications
            </label>
          </div>
          <Sheet.Close asChild>
            <Button>Save</Button>
          </Sheet.Close>
        </Sheet.Content>
      </Sheet.Portal>
    </Sheet.Root>
  );
}
