// SAFE: uses a ref to hold the latest selectedIndex value, ensuring the callback ref always has the current value

'use client'

import { useRef, useState } from 'react'

export default function MeasurePanel({ items }: { items: string[] }) {
  const [selectedIndex, setSelectedIndex] = useState(0)
  const indexRef = useRef(selectedIndex)
  indexRef.current = selectedIndex

  function measureRef(node: HTMLDivElement | null) {
    if (node) {
      console.log(`Width for item ${indexRef.current}:`, node.offsetWidth)
    }
  }

  return (
    <div>
      <button onClick={() => setSelectedIndex((i) => Math.max(0, i - 1))}>Prev</button>
      <button onClick={() => setSelectedIndex((i) => Math.min(items.length - 1, i + 1))}>Next</button>
      <div ref={measureRef} style={{ padding: 10 }}>
        {items[selectedIndex]}
      </div>
    </div>
  )
}
