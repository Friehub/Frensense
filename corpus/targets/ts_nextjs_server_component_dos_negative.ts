// SAFE: Maximum recursion depth is capped at a safe limit before rendering
// CVE: CVE-2025-55184

import { ReactNode } from 'react'

const MAX_DEPTH = 10

async function DeepTreeNode({ depth, children }: { depth: number; children: ReactNode }) {
  if (depth <= 0) return <>{children}</>

  return (
    <div>
      <DeepTreeNode depth={depth - 1}>
        <DeepTreeNode depth={depth - 1}>
          {children}
        </DeepTreeNode>
      </DeepTreeNode>
    </div>
  )
}

export default async function DeepTreePage({ searchParams }: { searchParams: Promise<{ depth?: string }> }) {
  const { depth } = await searchParams
  const safeDepth = Math.min(Number(depth) || 0, MAX_DEPTH)
  return <DeepTreeNode depth={safeDepth}>leaf</DeepTreeNode>
}
