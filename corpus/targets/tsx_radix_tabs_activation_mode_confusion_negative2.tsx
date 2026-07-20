// SAFE: Manual mode is documented with visible instructional text and all tabs have strong focus-visible styles

import * as Tabs from '@radix-ui/react-tabs';
import * as React from 'react';

export function ManualTabsWithInstructions() {
  return (
    <div>
      <p style={{ fontSize: 12, color: '#666', marginBottom: 4 }}>
        Use Tab to navigate between tabs. Press Enter or Space to activate a focused tab.
      </p>
      <Tabs.Root defaultValue="tab1" activationMode="manual">
        <Tabs.List>
          <Tabs.Trigger value="tab1" style={{ outlineOffset: 2 }}>Overview</Tabs.Trigger>
          <Tabs.Trigger value="tab2" style={{ outlineOffset: 2 }}>Details</Tabs.Trigger>
          <Tabs.Trigger value="tab3" style={{ outlineOffset: 2 }}>History</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="tab1">Overview content</Tabs.Content>
        <Tabs.Content value="tab2">Details content</Tabs.Content>
        <Tabs.Content value="tab3">History content</Tabs.Content>
      </Tabs.Root>
    </div>
  );
}
