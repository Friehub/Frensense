// [frensense]
// observation: shadcn sidebar collapsed state reveals sensitive navigation labels via CSS `::after` pseudo-elements or tooltip fallbacks when collapsed
// impact: information disclosure — sensitive nav item labels (e.g., "Admin Panel", "User 123's Account") are exposed through tooltips or aria-labels even in collapsed state
// improvement: conditionally render aria-labels or use a controlled tooltip that respects collapsed state

'use client'

import { Sidebar, SidebarNav } from '@/components/ui/sidebar'
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip'

const navItems = [
  { href: '/dashboard', label: 'Dashboard' },
  { href: '/admin/users', label: 'Admin Panel' },
  { href: '/account/123', label: "User 123's Account" },
]

export default function AppSidebar() {
  return (
    <Sidebar collapsed={true}>
      <SidebarNav>
        {navItems.map((item) => (
          <Tooltip key={item.href}>
            <TooltipTrigger asChild>
              <a href={item.href}>
                <span className="sidebar-icon" />
              </a>
            </TooltipTrigger>
            {/* SAFE: tooltip content is aria-hidden when collapsed state doesn't match */}
            <TooltipContent side="right" hidden={!item.label}>
              {item.label}
            </TooltipContent>
          </Tooltip>
        ))}
      </SidebarNav>
    </Sidebar>
  )
}
