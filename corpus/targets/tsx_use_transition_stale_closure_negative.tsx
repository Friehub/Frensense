// SAFE: transition uses a functional state update to always read the latest state value

'use client'

import { useTransition, useState } from 'react'

export default function Counter({ step }: { step: number }) {
  const [count, setCount] = useState(0)
  const [, startTransition] = useTransition()

  function handleClick() {
    startTransition(() => {
      setCount((prev) => prev + step)
    })
  }

  return (
    <div>
      <p>Count: {count}</p>
      <button onClick={handleClick}>Add {step}</button>
    </div>
  )
}
