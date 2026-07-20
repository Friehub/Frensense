// SAFE: destructured value is sanitized before portaling
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
  const { text } = { text: DOMPurify.sanitize(message) }
  return createPortal(<div dangerouslySetInnerHTML={{ __html: text }} />, containerRef.current)
}
