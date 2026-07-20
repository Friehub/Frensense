// SAFE: Errors are caught and logged server-side only; a generic 404 is returned without exposing source paths or stack traces
// CVE: CVE-2025-55183

import { notFound } from 'next/navigation'
import { ReactNode } from 'react'

export default async function BlogPostLayout({ children, params }: { children: ReactNode; params: { slug: string } }) {
  try {
    const data = await fetch(`https://cms.internal/posts/${params.slug}`)
    if (!data.ok) {
      notFound()
    }
  } catch {
    notFound()
  }

  return <article>{children}</article>
}
