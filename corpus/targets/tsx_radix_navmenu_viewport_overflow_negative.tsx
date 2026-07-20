// SAFE: The Viewport uses responsive max-width and overflow-auto, and content widths are constrained to prevent clipping

import * as NavigationMenu from '@radix-ui/react-navigation-menu';
import { ChevronDownIcon } from '@radix-ui/react-icons';

export function MainNav() {
  return (
    <NavigationMenu.Root>
      <NavigationMenu.List>
        <NavigationMenu.Item>
          <NavigationMenu.Trigger>
            Products <ChevronDownIcon />
          </NavigationMenu.Trigger>
          <NavigationMenu.Content>
            <ul style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8, minWidth: 200, maxWidth: 360 }}>
              <li><a href="/products/widget">Widget</a></li>
              <li><a href="/products/gadget">Gadget</a></li>
              <li><a href="/products/tool">Tool</a></li>
              <li><a href="/products/device">Device</a></li>
              <li><a href="/products/component">Component</a></li>
              <li><a href="/products/accessory">Accessory</a></li>
            </ul>
          </NavigationMenu.Content>
        </NavigationMenu.Item>
        <NavigationMenu.Item>
          <NavigationMenu.Trigger>
            Resources <ChevronDownIcon />
          </NavigationMenu.Trigger>
          <NavigationMenu.Content>
            <ul style={{ minWidth: 160, maxWidth: 300 }}>
              <li><a href="/docs">Docs</a></li>
              <li><a href="/blog">Blog</a></li>
              <li><a href="/support">Support</a></li>
            </ul>
          </NavigationMenu.Content>
        </NavigationMenu.Item>
      </NavigationMenu.List>
      <NavigationMenu.Viewport style={{ maxWidth: 'min(90vw, 420px)', overflow: 'auto' }} />
    </NavigationMenu.Root>
  );
}
