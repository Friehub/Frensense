// [frensense]
// observation: A NavigationMenu does not set `viewport` overflow handling or responsive width constraints, causing the viewport to clip menu content on small viewports, rendering submenu items unreachable.
// impact: On screens narrower than the navigation menu's content width, the viewport overflows and clips menu items. Users on mobile or zoomed-in viewports cannot access submenu items, effectively hiding navigation options and making the application partially unusable.
// improvement: Set `NavigationMenu.Viewport` with proper overflow styles and responsive width, or use `NavigationMenu.Content` with `onPointerEnter` for responsive submenus.

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
            <ul style={{ width: 400, display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
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
            <ul style={{ width: 400 }}>
              <li><a href="/docs">Docs</a></li>
              <li><a href="/blog">Blog</a></li>
              <li><a href="/support">Support</a></li>
            </ul>
          </NavigationMenu.Content>
        </NavigationMenu.Item>
      </NavigationMenu.List>
      <NavigationMenu.Viewport style={{ width: 420 }} />
    </NavigationMenu.Root>
  );
}
