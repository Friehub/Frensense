// SAFE: portal renders text content
'use client'
import { createPortal } from 'react'
import { useRef } from 'react'
export default function ToastPortal({ message }: { message: string }) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  if (!containerRef.current) {
    containerRef.current = document.createElement('div')
    document.body.appendChild(containerRef.current)
  }
  return createPortal(<div>{`${message}`}</div>, containerRef.current)
}
