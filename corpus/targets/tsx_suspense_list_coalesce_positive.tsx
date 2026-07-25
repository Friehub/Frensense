// [frensense]
// observation: `<SuspenseList>` with `revealOrder="together"` incorrectly coalesces multiple Suspense boundaries, hiding individual loading states until all are ready
// impact: poor UX — users see nothing until the slowest child resolves, even when faster siblings could have been shown progressively
// improvement: use `revealOrder="forwards"` or `revealOrder="backwards"` for progressive loading, or avoid SuspenseList when independent loading is acceptable
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium

'use client'

import { Suspense, SuspenseList } from 'react'
import type { ReactNode } from 'react'

function SlowComponent({ name, delay }: { name: string; delay: number }) {
  return new Promise<ReactNode>((resolve) => {
    setTimeout(() => resolve(<div>{name} loaded</div>), delay)
  }) as unknown as ReactNode
}

export default function Dashboard() {
  return (
    <SuspenseList revealOrder="together">
      <Suspense fallback={<div>Loading widget A...</div>}>
        <SlowComponent name="Widget A" delay={3000} />
      </Suspense>
      <Suspense fallback={<div>Loading widget B...</div>}>
        <SlowComponent name="Widget B" delay={1000} />
      </Suspense>
    </SuspenseList>
  )
}
