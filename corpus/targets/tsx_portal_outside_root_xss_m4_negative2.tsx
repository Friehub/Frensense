// SAFE: helper sanitizes before returning
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
import DOMPurify from 'dompurify'
function transform(x: string): string { return DOMPurify.sanitize(x); }
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  const msg = transform(message)
  return createPortal(<div dangerouslySetInnerHTML={{ __html: msg }} />, containerRef.current)
}
