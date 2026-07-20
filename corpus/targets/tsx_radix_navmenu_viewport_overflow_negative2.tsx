// SAFE: Uses responsive styling with CSS clamp and media queries to ensure the viewport adapts to all screen widths

import * as NavigationMenu from '@radix-ui/react-navigation-menu';
import { ChevronDownIcon } from '@radix-ui/react-icons';

const viewportStyle: React.CSSProperties = {
  width: 'clamp(200px, 80vw, 420px)',
  overflow: 'auto',
  maxHeight: '80vh',
};

export function MainNav() {
  return (
    <NavigationMenu.Root>
      <NavigationMenu.List>
        <NavigationMenu.Item>
          <NavigationMenu.Trigger>
            Products <ChevronDownIcon />
          </NavigationMenu.Trigger>
          <NavigationMenu.Content>
            <ul style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(120px, 1fr))', gap: 8 }}>
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
            <ul style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
              <li><a href="/docs">Docs</a></li>
              <li><a href="/blog">Blog</a></li>
              <li><a href="/support">Support</a></li>
            </ul>
          </NavigationMenu.Content>
        </NavigationMenu.Item>
      </NavigationMenu.List>
      <NavigationMenu.Viewport style={viewportStyle} />
    </NavigationMenu.Root>
  );
}
