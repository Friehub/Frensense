// SAFE: uses AbortController to clean up the event listener, avoiding the need for removeEventListener reference

'use client'

import { createPortal, useEffect, useRef } from 'react'

export default function ModalPortal({ children }: { children: React.ReactNode }) {
  const containerRef = useRef<HTMLDivElement | null>(null)

  useEffect(() => {
    const container = document.createElement('div')
    document.body.appendChild(container)
    containerRef.current = container

    const controller = new AbortController()

    document.addEventListener('keydown', (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        console.log('Modal dismissed')
      }
    }, { signal: controller.signal })

    return () => {
      controller.abort()
      document.body.removeChild(container)
      containerRef.current = null
    }
  }, [])

  if (!containerRef.current) return null

  return createPortal(children, containerRef.current)
}
