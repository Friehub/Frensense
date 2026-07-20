// SAFE: sanitizes user HTML before rendering in the portal using DOMPurify

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

  const sanitized = DOMPurify.sanitize(message)

  return createPortal(
    <div dangerouslySetInnerHTML={{ __html: sanitized }} />,
    containerRef.current,
  )
}
