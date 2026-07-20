// SAFE: transition uses a ref to capture the latest prop value instead of the closure value

'use client'

import { useTransition, useState, useRef } from 'react'

export default function Counter({ step }: { step: number }) {
  const [count, setCount] = useState(0)
  const stepRef = useRef(step)
  stepRef.current = step
  const [, startTransition] = useTransition()

  function handleClick() {
    startTransition(() => {
      setCount((prev) => prev + stepRef.current)
    })
  }

  return (
    <div>
      <p>Count: {count}</p>
      <button onClick={handleClick}>Add {step}</button>
    </div>
  )
}
