// SAFE: Destructive actions open a confirmation dialog before executing, preventing accidental account deletion

import * as Menubar from '@radix-ui/react-menubar';
import * as AlertDialog from '@radix-ui/react-alert-dialog';
import { useNavigate } from 'react-router-dom';
import { useState } from 'react';

export function AccountMenubar() {
  const navigate = useNavigate();
  const [deleteConfirmOpen, setDeleteConfirmOpen] = useState(false);

  return (
    <>
      <Menubar.Root>
        <Menubar.Menu>
          <Menubar.Trigger>Account</Menubar.Trigger>
          <Menubar.Portal>
            <Menubar.Content>
              <Menubar.Item onSelect={() => navigate('/account/settings')}>
                Settings
              </Menubar.Item>
              <Menubar.Item onSelect={() => navigate('/account/billing')}>
                Billing
              </Menubar.Item>
              <Menubar.Separator />
              <Menubar.Item onSelect={() => setDeleteConfirmOpen(true)}>
                Delete Account
              </Menubar.Item>
            </Menubar.Content>
          </Menubar.Portal>
        </Menubar.Menu>
      </Menubar.Root>
      <AlertDialog.Root open={deleteConfirmOpen} onOpenChange={setDeleteConfirmOpen}>
        <AlertDialog.Portal>
          <AlertDialog.Overlay />
          <AlertDialog.Content>
            <AlertDialog.Title>Delete Account?</AlertDialog.Title>
            <AlertDialog.Description>This action is irreversible. All data will be permanently lost.</AlertDialog.Description>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action onClick={() => fetch('/api/account/delete', { method: 'POST' })}>
              Confirm Delete
            </AlertDialog.Action>
          </AlertDialog.Content>
        </AlertDialog.Portal>
      </AlertDialog.Root>
    </>
  );
}
