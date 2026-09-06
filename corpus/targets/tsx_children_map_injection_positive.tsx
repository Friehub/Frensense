// [frensense]
// observation: `React.Children.map` is used over children that contain user content, and the mapped output is rendered without sanitization
// impact: XSS — user-controlled child content containing malicious HTML/JS is rendered into the DOM
// improvement: sanitize rendered content or use React's built-in text escaping by passing children as text nodes

'use client'

import { Children, cloneElement, isValidElement } from 'react'
import type { ReactNode } from 'react'

interface WrapperProps {
  children?: ReactNode
}

export default function BadgeWrapper({ children }: WrapperProps) {
  const enhanced = Children.map(children, (child) => {
    if (isValidElement(child)) {
      return cloneElement(child, { className: 'badge' } as Record<string, unknown>)
    }
    return child
  })

  return <div className="badge-container">{enhanced}</div>
}

export function UserBadge({ userName }: { userName: string }) {
  return <BadgeWrapper>{userName}</BadgeWrapper>
}
