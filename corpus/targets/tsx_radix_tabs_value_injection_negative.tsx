// SAFE: The hash value is validated against a whitelist of allowed tab values before being used

import * as Tabs from '@radix-ui/react-tabs';
import * as React from 'react';

const ALLOWED_TABS = ['account', 'billing', 'settings'] as const;
type AllowedTab = typeof ALLOWED_TABS[number];

function isValidTab(value: string): value is AllowedTab {
  return ALLOWED_TABS.includes(value as AllowedTab);
}

export function HashControlledTabs() {
  const hash = typeof window !== 'undefined' ? window.location.hash.replace('#', '') : '';
  const initialTab = isValidTab(hash) ? hash : 'account';
  const [activeTab, setActiveTab] = React.useState<AllowedTab>(initialTab);

  return (
    <Tabs.Root value={activeTab} onValueChange={(v) => { if (isValidTab(v)) setActiveTab(v); }}>
      <Tabs.List>
        <Tabs.Trigger value="account">Account</Tabs.Trigger>
        <Tabs.Trigger value="billing">Billing</Tabs.Trigger>
        <Tabs.Trigger value="settings">Settings</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="account">Account details</Tabs.Content>
      <Tabs.Content value="billing">Billing info</Tabs.Content>
      <Tabs.Content value="settings">Settings panel</Tabs.Content>
    </Tabs.Root>
  );
}
