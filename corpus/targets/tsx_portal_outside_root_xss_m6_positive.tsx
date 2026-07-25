// [frensense]
// observation: User-controlled message is concatenated before portal dangerouslySetInnerHTML.
// impact: XSS — concatenation prefix does not sanitize embedded HTML.
// improvement: Sanitize concatenated result or avoid dangerouslySetInnerHTML.
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
  const html = "<div>" + message + "</div>"
  return createPortal(<div dangerouslySetInnerHTML={{ __html: html }} />, containerRef.current)
}
