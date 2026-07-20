// [frensense]
// observation: User-controlled message is injected via template literal into portal dangerouslySetInnerHTML.
// impact: XSS — template literal does not sanitize HTML.
// improvement: Sanitize template output or avoid dangerouslySetInnerHTML.
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div dangerouslySetInnerHTML={{ __html: `${message}` }} />, containerRef.current)
}
