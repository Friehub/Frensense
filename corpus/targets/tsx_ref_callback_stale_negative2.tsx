// SAFE: uses a regular ref (useRef) instead of a callback ref, logging the value inside useEffect instead

'use client'

import { useEffect, useRef, useState } from 'react'

export default function MeasurePanel({ items }: { items: string[] }) {
  const [selectedIndex, setSelectedIndex] = useState(0)
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (containerRef.current) {
      console.log(`Width for item ${selectedIndex}:`, containerRef.current.offsetWidth)
    }
  }, [selectedIndex])

  return (
    <div>
      <button onClick={() => setSelectedIndex((i) => Math.max(0, i - 1))}>Prev</button>
      <button onClick={() => setSelectedIndex((i) => Math.min(items.length - 1, i + 1))}>Next</button>
      <div ref={containerRef} style={{ padding: 10 }}>
        {items[selectedIndex]}
      </div>
    </div>
  )
}
