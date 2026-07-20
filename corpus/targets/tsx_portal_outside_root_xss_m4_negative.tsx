// SAFE: portal renders text content instead of dangerouslySetInnerHTML
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
  return createPortal(<div>{msg}</div>, containerRef.current)
}
