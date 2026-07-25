// [frensense]
// observation: A server component recursively renders deeply nested child components based on user-controlled depth parameter, creating an exponential rendering tree.
// impact: An attacker can supply a large depth value causing the server to exhaust memory and crash, leading to denial of service (CVE-2025-55184 variant).
// improvement: Impose a maximum recursion depth limit, or use an iterative approach instead of recursive component rendering.
// cwe: CWE-400
// cvss: 7.5
// owasp: 
// severity: High
// CVE: CVE-2025-55184

import { ReactNode } from 'react'

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
  return <DeepTreeNode depth={Number(depth) || 0}>leaf</DeepTreeNode>
}
