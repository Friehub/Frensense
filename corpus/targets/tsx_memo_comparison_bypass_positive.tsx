// [frensense]
// observation: `React.memo` with a comparison function that always returns `false` forces the component to re-render on every parent render, bypassing memoization entirely
// impact: performance degradation — expensive component re-renders every time, negating the benefit of `React.memo`
// improvement: use a correct comparison function (or none for shallow comparison)

'use client'

import { memo } from 'react'

interface ExpensiveChartProps {
  data: number[]
  title: string
}

function ExpensiveChart({ data, title }: ExpensiveChartProps) {
  return (
    <div>
      <h3>{title}</h3>
      <svg>{data.map((v, i) => <circle key={i} cx={i * 10} cy={100 - v} r={5} />)}</svg>
    </div>
  )
}

export default memo(ExpensiveChart, () => false)
