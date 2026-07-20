// SAFE: sanitization applied before portal render
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const a = message
  const b = DOMPurify.sanitize(a)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: b }} />, containerRef.current)
}
