// [frensense]
// observation: User-controlled message flows through two assignments before portal render with dangerouslySetInnerHTML.
// impact: XSS via multi-hop taint into portal.
// improvement: Sanitize or avoid dangerouslySetInnerHTML.
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
  const a = message
  const b = a
  return createPortal(<div dangerouslySetInnerHTML={{ __html: b }} />, containerRef.current)
}
