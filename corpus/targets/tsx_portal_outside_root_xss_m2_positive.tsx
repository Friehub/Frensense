// [frensense]
// observation: User-controlled message is assigned to an intermediate variable before being rendered in a portal with dangerouslySetInnerHTML.
// impact: XSS — portaled content escapes React's DOM control.
// improvement: Sanitize the intermediate variable or avoid dangerouslySetInnerHTML in portals.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = message
  return createPortal(<div dangerouslySetInnerHTML={{ __html: msg }} />, containerRef.current)
}
