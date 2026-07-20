// SAFE: Uses activationMode: 'automatic' so arrow key navigation immediately activates tabs, matching standard keyboard expectations

import * as Tabs from '@radix-ui/react-tabs';

export function AutomaticTabs() {
  return (
    <Tabs.Root defaultValue="tab1" activationMode="automatic">
      <Tabs.List>
        <Tabs.Trigger value="tab1">Overview</Tabs.Trigger>
        <Tabs.Trigger value="tab2">Details</Tabs.Trigger>
        <Tabs.Trigger value="tab3">History</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="tab1">Overview content</Tabs.Content>
      <Tabs.Content value="tab2">Details content</Tabs.Content>
      <Tabs.Content value="tab3">History content</Tabs.Content>
    </Tabs.Root>
  );
}
