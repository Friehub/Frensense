// SAFE: uses a global error boundary to handle 404s without revealing routing info

import { ReactNode } from 'react'

export default async function ProjectLayout({ children }: { children: ReactNode }) {
  return <div>{children}</div>
}
