// [frensense]
// observation: A Tabs component uses the URL hash fragment (window.location.hash) to set the `value` prop without validation, allowing an attacker to activate any tab value via a crafted link.
// impact: An attacker can craft a URL like `https://app.example.com/profile#settings-admin` that activates a hidden admin tab, or `#billing-cancel` that auto-navigates to a destructive action tab. The user is phished into clicking the link and the tab changes without their consent, potentially triggering destructive operations or exposing sensitive panels.
// improvement: Validate the hash value against a whitelist of known tab values before passing it to the Tabs component.

import * as Tabs from '@radix-ui/react-tabs';
import * as React from 'react';

export function HashControlledTabs() {
  const hash = typeof window !== 'undefined' ? window.location.hash.replace('#', '') : 'account';
  const [activeTab, setActiveTab] = React.useState(hash);

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
