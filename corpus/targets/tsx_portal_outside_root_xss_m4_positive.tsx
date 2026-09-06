// [frensense]
// observation: User-controlled message passes through a helper that does not sanitize before portaling with dangerouslySetInnerHTML.
// impact: XSS — helper returns unsanitized HTML to portal.
// improvement: Sanitize helper output or avoid dangerouslySetInnerHTML.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
function transform(x: string): string { return x; }
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = transform(message)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: msg }} />, containerRef.current)
}
