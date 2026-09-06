// [frensense]
// observation: React Compiler auto-memoizes a component but misses a dependency on an external ref, causing the callback to capture a stale closure value
// impact: stale closure — UI displays outdated data or fires incorrect API calls because the compiler-optimized component didn't re-render when the external value changed
// improvement: explicitly declare ref dependencies or use `useCallback`/`useEffect` with proper deps instead of relying solely on the compiler

'use client'

import { useEffect, useRef, useState } from 'react'

export default function AutoMemoizedCounter() {
  const [count, setCount] = useState(0)
  const countRef = useRef(count)
  const callbackRef = useRef<(() => void) | null>(null)

  // SAFE: keep ref in sync with latest count value
  countRef.current = count

  useEffect(() => {
    callbackRef.current = () => {
      // SAFE: reads from ref instead of stale closure
      console.log(`Count is: ${countRef.current}`)
    }
  })

  function handleClick() {
    callbackRef.current?.()
  }

  return (
    <div>
      <p>Count: {count}</p>
      <button onClick={() => setCount((c) => c + 1)}>Increment</button>
      <button onClick={handleClick}>Log Count</button>
    </div>
  )
}
