// SAFE: Even with modal: false, a pointer-events overlay blocks interaction with the background content

import * as Sheet from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';

export function SettingsSheet() {
  return (
    <Sheet.Root modal={false}>
      <Sheet.Trigger asChild>
        <Button>Open Settings</Button>
      </Sheet.Trigger>
      <Sheet.Portal>
        <div style={{ position: 'fixed', inset: 0, background: 'rgba(0,0,0,0.3)', pointerEvents: 'auto' }} />
        <Sheet.Content style={{ position: 'relative', zIndex: 10 }}>
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
