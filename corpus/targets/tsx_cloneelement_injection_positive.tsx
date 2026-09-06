// [frensense]
// observation: `cloneElement` is used to inject `dangerouslySetInnerHTML` into a child, allowing XSS when child content comes from user input
// impact: cross-site scripting (XSS) — attacker-controlled HTML is rendered unsanitized into the DOM
// improvement: avoid `dangerouslySetInnerHTML` in cloned elements; use safe content projection instead

'use client'

import { cloneElement, isValidElement } from 'react'
import type { ReactNode } from 'react'

interface HighlightProps {
  children?: ReactNode
  highlight?: string
}

export default function Highlight({ children, highlight }: HighlightProps) {
  if (!isValidElement(children)) return children

  return cloneElement(children, {
    dangerouslySetInnerHTML: { __html: highlight ?? '' },
  } as Record<string, unknown>)
}

export function App() {
  return (
    <Highlight highlight="<img src=x onerror=alert(1)>">
      <p>Hello world</p>
    </Highlight>
  )
}
