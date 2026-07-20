// [frensense]
// observation: `React.Children.map` is used over children that contain user content, and the mapped output is rendered without sanitization
// impact: XSS — user-controlled child content containing malicious HTML/JS is rendered into the DOM
// improvement: sanitize rendered content or use React's built-in text escaping by passing children as text nodes

'use client'

import { Children, isValidElement } from 'react'
import type { ReactNode } from 'react'

interface WrapperProps {
  children?: ReactNode
}

export default function BadgeWrapper({ children }: WrapperProps) {
  const count = Children.count(children)

  // SAFE: only count is used, children are not manipulated or rendered directly
  return <div className="badge-container" data-count={count}>{children}</div>
}

export function UserBadge({ userName }: { userName: string }) {
  return <BadgeWrapper>{userName}</BadgeWrapper>
}
