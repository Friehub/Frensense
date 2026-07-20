// SAFE: uses a redirect to a generic error page instead of notFound

import { redirect } from 'next/navigation'
import { ReactNode } from 'react'

export default async function ProjectLayout({ children, params }: { children: ReactNode; params: { id: string } }) {
  const res = await fetch(`https://api.internal/projects/${params.id}`)
  if (res.status === 404) redirect('/error')
  const project = await res.json()
  return (
    <div>
      <h1>{project.name}</h1>
      {children}
    </div>
  )
}
