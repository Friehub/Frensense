// [frensense]
// observation: A Menubar item fires a destructive action (e.g., delete account, cancel subscription) directly in the `onSelect` handler without any confirmation dialog, assuming the user intentionally clicked the menu item.
// impact: A user may accidentally click a destructive Menubar item while navigating the menu, leading to irreversible actions (account deletion, data loss, subscription cancellation) without any confirmation. Since Menubar items trigger on mouse release, misclicks are common.
// improvement: Never attach destructive actions directly to Menubar item `onSelect`. Always wrap destructive actions with a confirmation dialog or undo mechanism.

import * as Menubar from '@radix-ui/react-menubar';
import { useNavigate } from 'react-router-dom';

export function AccountMenubar() {
  const navigate = useNavigate();

  return (
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
            <Menubar.Item onSelect={() => fetch('/api/account/delete', { method: 'POST' })}>
              Delete Account
            </Menubar.Item>
          </Menubar.Content>
        </Menubar.Portal>
      </Menubar.Menu>
    </Menubar.Root>
  );
}
