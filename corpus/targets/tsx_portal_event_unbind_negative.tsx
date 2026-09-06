// SAFE: useEffect returns a cleanup function that removes the event listener and portal DOM node

'use client'

import { createPortal, useEffect, useRef } from 'react'

export default function ModalPortal({ children }: { children: React.ReactNode }) {
  const containerRef = useRef<HTMLDivElement | null>(null)

  useEffect(() => {
    const container = document.createElement('div')
    document.body.appendChild(container)
    containerRef.current = container

    function handleKeyDown(e: KeyboardEvent) {
      if (e.key === 'Escape') {
        console.log('Modal dismissed')
      }
    }

    document.addEventListener('keydown', handleKeyDown)

    return () => {
      document.removeEventListener('keydown', handleKeyDown)
      document.body.removeChild(container)
      containerRef.current = null
    }
  }, [])

  if (!containerRef.current) return null

  return createPortal(children, containerRef.current)
}
