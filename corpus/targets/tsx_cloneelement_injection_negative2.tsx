// [frensense]
// observation: `cloneElement` is used to inject `dangerouslySetInnerHTML` into a child, allowing XSS when child content comes from user input
// impact: cross-site scripting (XSS) — attacker-controlled HTML is rendered unsanitized into the DOM
// improvement: avoid `dangerouslySetInnerHTML` in cloned elements; use safe content projection instead

'use client'

import { isValidElement } from 'react'
import type { ReactNode } from 'react'

interface HighlightProps {
  children?: ReactNode
  highlight?: string
}

export default function Highlight({ children, highlight }: HighlightProps) {
  // SAFE: no cloneElement or dangerouslySetInnerHTML; children are rendered as-is
  return <span data-highlight={highlight}>{children}</span>
}

export function App() {
  return (
    <Highlight highlight="safe text">
      <p>Hello world</p>
    </Highlight>
  )
}
