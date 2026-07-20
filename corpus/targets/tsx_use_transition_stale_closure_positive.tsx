// [frensense]
// observation: `useTransition` wraps a state update that reads from a stale closure variable (e.g., a prop or state that was captured when the transition was queued)
// impact: the transition updates state using a stale value, causing the UI to reflect outdated data or revert a more recent change
// improvement: read the latest value inside the transition via a ref or use functional state updates

'use client'

import { useTransition, useState } from 'react'

export default function Counter({ step }: { step: number }) {
  const [count, setCount] = useState(0)
  const [, startTransition] = useTransition()

  function handleClick() {
    startTransition(() => {
      setCount(count + step)
    })
  }

  return (
    <div>
      <p>Count: {count}</p>
      <button onClick={handleClick}>Add {step}</button>
    </div>
  )
}
