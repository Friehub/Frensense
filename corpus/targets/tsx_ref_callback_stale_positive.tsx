// [frensense]
// observation: Callback ref captures a stale closure variable — when the ref callback fires, it references an outdated value of a state or prop
// impact: wrong element reference or stale data is used in the callback, leading to incorrect DOM measurements or event binding
// improvement: use a ref to capture the latest value, or use `useCallback` with proper deps

'use client'

import { useState } from 'react'

export default function MeasurePanel({ items }: { items: string[] }) {
  const [selectedIndex, setSelectedIndex] = useState(0)

  function measureRef(node: HTMLDivElement | null) {
    if (node) {
      console.log(`Width for item ${selectedIndex}:`, node.offsetWidth)
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
