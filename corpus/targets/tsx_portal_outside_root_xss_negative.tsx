// SAFE: uses text content instead of dangerouslySetInnerHTML to render the message in the portal

'use client'

import { createPortal } from 'react'
import { useRef } from 'react'

export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)

  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }

  return createPortal(<div>{message}</div>, containerRef.current)
}
