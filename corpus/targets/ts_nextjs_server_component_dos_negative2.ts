// SAFE: The recursive component is replaced by an iterative flat list with no unbounded recursion
// CVE: CVE-2025-55184

import { ReactNode } from 'react'

const MAX_DEPTH = 10

function FlatTree({ depth, children }: { depth: number; children: ReactNode }) {
  const items: ReactNode[] = []
  for (let i = 0; i < Math.min(depth, MAX_DEPTH); i++) {
    items.push(<div key={i} style={{ marginLeft: 20 }}>{i === depth - 1 ? children : '...'}</div>)
  }
  return <>{items}</>
}

export default async function DeepTreePage({ searchParams }: { searchParams: Promise<{ depth?: string }> }) {
  const { depth } = await searchParams
  return <FlatTree depth={Number(depth) || 0}>leaf</FlatTree>
}
