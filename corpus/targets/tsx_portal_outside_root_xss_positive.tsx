// [frensense]
// observation: `createPortal` renders user-controlled HTML content (from a dangerouslySetInnerHTML prop) outside the React root into document.body
// impact: XSS attack — user-provided HTML is injected into the DOM outside React's control and sanitization, enabling arbitrary script execution
// improvement: sanitize user HTML before portaling, or avoid dangerouslySetInnerHTML in portal content

'use client'

import { createPortal } from 'react'
import { useRef } from 'react'

export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)

  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }

  return createPortal(
    <div dangerouslySetInnerHTML={{ __html: message }} />,
    containerRef.current,
  )
}
