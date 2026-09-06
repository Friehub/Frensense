// SAFE: removes the ref altogether and uses a state variable for tracking renders

'use client'

import { useState } from 'react'

export default function Counter() {
  const [count, setCount] = useState(0)
  const [renders, setRenders] = useState(0)

  useState(() => {
    setRenders((r) => r + 1)
  })

  return (
    <div>
      <p>Count: {count}</p>
      <p>Renders: {renders}</p>
      <button onClick={() => setCount((c) => c + 1)}>Increment</button>
    </div>
  )
}
