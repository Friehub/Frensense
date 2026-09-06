// [frensense]
// observation: `ref.current` is mutated directly during the render phase (inside the component body, not in useEffect)
// impact: side effect during render causes inconsistent UI state and breaks React's pure render guarantee — may lead to tearing or infinite re-renders
// improvement: move ref mutations to useEffect or event handlers

'use client'

import { useRef, useState } from 'react'

export default function Counter() {
  const [count, setCount] = useState(0)
  const rendersRef = useRef(0)

  rendersRef.current = rendersRef.current + 1

  return (
    <div>
      <p>Count: {count}</p>
      <p>Renders: {rendersRef.current}</p>
      <button onClick={() => setCount((c) => c + 1)}>Increment</button>
    </div>
  )
}
