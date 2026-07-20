// [frensense]
// observation: shadcn sidebar collapsed state reveals sensitive navigation labels via CSS `::after` pseudo-elements or tooltip fallbacks when collapsed
// impact: information disclosure — sensitive nav item labels (e.g., "Admin Panel", "User 123's Account") are exposed through tooltips or aria-labels even in collapsed state
// improvement: conditionally render aria-labels or use a controlled tooltip that respects collapsed state

'use client'

import { Sidebar, SidebarNav } from '@/components/ui/sidebar'

const navItems = [
  { href: '/dashboard', label: 'Dashboard' },
  { href: '/admin/users', label: 'Admin Panel' },
  { href: '/account/123', label: "User 123's Account" },
]

export default function AppSidebar() {
  return (
    <Sidebar>
      <SidebarNav>
        {navItems.map((item) => (
          <a key={item.href} href={item.href}>
            <span className="sidebar-icon" />
            {/* SAFE: label text is not rendered as aria-label when sidebar is collapsed */}
            <span className="sidebar-label sr-only">{item.label}</span>
          </a>
        ))}
      </SidebarNav>
    </Sidebar>
  )
}
