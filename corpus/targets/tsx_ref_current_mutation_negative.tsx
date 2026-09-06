// SAFE: ref mutation is moved to useEffect, keeping the render phase pure

'use client'

import { useEffect, useRef, useState } from 'react'

export default function Counter() {
  const [count, setCount] = useState(0)
  const rendersRef = useRef(0)

  useEffect(() => {
    rendersRef.current = rendersRef.current + 1
  })

  return (
    <div>
      <p>Count: {count}</p>
      <p>Renders: {rendersRef.current}</p>
      <button onClick={() => setCount((c) => c + 1)}>Increment</button>
    </div>
  )
}
