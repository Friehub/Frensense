// SAFE: Destructive menu items are visually separated and disabled by default until the user confirms via a secondary action

import * as Menubar from '@radix-ui/react-menubar';
import { useState } from 'react';

export function AccountMenubar() {
  const [confirmMode, setConfirmMode] = useState(false);

  return (
    <Menubar.Root>
      <Menubar.Menu>
        <Menubar.Trigger>Account</Menubar.Trigger>
        <Menubar.Portal>
          <Menubar.Content>
            <Menubar.Item onSelect={() => alert('Navigate to settings')}>
              Settings
            </Menubar.Item>
            <Menubar.Separator />
            {confirmMode ? (
              <>
                <Menubar.Item onSelect={() => { fetch('/api/account/delete', { method: 'POST' }); setConfirmMode(false); }}>
                  Confirm Delete (irreversible)
                </Menubar.Item>
                <Menubar.Item onSelect={() => setConfirmMode(false)}>
                  Cancel
                </Menubar.Item>
              </>
            ) : (
              <Menubar.Item onSelect={() => setConfirmMode(true)}>
                Delete Account
              </Menubar.Item>
            )}
          </Menubar.Content>
        </Menubar.Portal>
      </Menubar.Menu>
    </Menubar.Root>
  );
}
