// [frensense]
// observation: A Tabs component uses `activationMode: 'manual'` but does not provide keyboard instruction or visual focus indicators for tab activation, causing keyboard users to be unable to activate tabs via arrow keys (common expectation from `automatic` mode).
// impact: Keyboard-only users who expect tabs to activate on arrow key navigation (automatic mode) find themselves stuck. They press ArrowRight expecting the next tab to show, but nothing happens because manual mode requires Enter/Space to activate. This creates a keyboard trap and violates WCAG 2.1.1 (Keyboard).
// improvement: Use `activationMode: 'automatic'` for typical tab panels, or if manual is required, provide clear instructions and ensure all tabs have visible focus styles.

import * as Tabs from '@radix-ui/react-tabs';

export function ManualTabs() {
  return (
    <Tabs.Root defaultValue="tab1" activationMode="manual">
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
