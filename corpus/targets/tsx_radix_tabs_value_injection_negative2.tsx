// SAFE: The tabs do not read from window.location.hash at all — tab state is local only, preventing URL-based tab manipulation

import * as Tabs from '@radix-ui/react-tabs';
import * as React from 'react';

export function LocalControlledTabs() {
  const [activeTab, setActiveTab] = React.useState('account');

  return (
    <Tabs.Root value={activeTab} onValueChange={setActiveTab}>
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
