// [frensense]
// observation: calling `notFound()` inside a layout reveals the existence of internal route segments or resource IDs in the URL
// impact: attackers can probe route existence by observing 404 vs 200 responses; internal routing structure is exposed
// improvement: use a generic error boundary or redirect rather than notFound in layouts

import { notFound } from 'next/navigation'
import { ReactNode } from 'react'

export default async function ProjectLayout({ children, params }: { children: ReactNode; params: { id: string } }) {
  const res = await fetch(`https://api.internal/projects/${params.id}`)
  if (res.status === 404) notFound()
  const project = await res.json()
  return (
    <div>
      <h1>{project.name}</h1>
      {children}
    </div>
  )
}
