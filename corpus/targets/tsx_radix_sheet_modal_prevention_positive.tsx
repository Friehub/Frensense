// [frensense]
// observation: A Sheet component uses `modal: false` (or the default non-modal behavior in some implementations), allowing users to interact with background content while the sheet is open, which can lead to inconsistent state or unintended actions.
// impact: While the sheet displays important information or a form, the user can click buttons, follow links, or submit forms on the main page. This can cause data loss (the sheet's state is discarded), double submissions, or security issues where the sheet's context is bypassed.
// improvement: Use `modal: true` to prevent background interaction while the sheet is open, or dim the background and disable pointer events on the underlying content.

import * as Sheet from '@radix-ui/react-dialog';
import { Button } from '@/components/ui/button';

export function SettingsSheet() {
  return (
    <Sheet.Root modal={false}>
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
