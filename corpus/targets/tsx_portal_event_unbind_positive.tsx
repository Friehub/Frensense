// [frensense]
// observation: Portal adds a global event listener (e.g., mousedown, keydown) in a useEffect but never removes it on unmount
// impact: memory leak — the event listener and portal DOM node persist after the component unmounts, causing stale callbacks and resource buildup
// improvement: return a cleanup function from useEffect that removes the event listener and portal DOM node

'use client'

import { createPortal, useEffect, useRef } from 'react'

export default function ModalPortal({ children }: { children: React.ReactNode }) {
  const containerRef = useRef<HTMLDivElement | null>(null)

  useEffect(() => {
    if (!containerRef.current) {
      containerRef.current = document.createElement('div')
      document.body.appendChild(containerRef.current)
    }

    function handleKeyDown(e: KeyboardEvent) {
      if (e.key === 'Escape') {
        console.log('Modal dismissed')
      }
    }

    document.addEventListener('keydown', handleKeyDown)
  }, [])

  if (!containerRef.current) return null

  return createPortal(children, containerRef.current)
}
