// [frensense]
// observation: `notFound()` is thrown inside a layout that performs server-side rendering with detailed error serialization enabled, causing the raw component source code or stack trace to be included in the 404 response body.
// impact: Attackers can probe non-existent routes and receive source code fragments or internal file paths, aiding reconnaissance and enabling further attacks (CVE-2025-55183 variant).
// improvement: Use a custom error boundary that catches the error, logs details internally, and returns a generic 404 page without exposing internal information.
// CVE: CVE-2025-55183

import { notFound } from 'next/navigation'
import { ReactNode } from 'react'

export default async function BlogPostLayout({ children, params }: { children: ReactNode; params: { slug: string } }) {
  const data = await fetch(`https://cms.internal/posts/${params.slug}`).catch(() => null)

  if (!data) {
    console.error(`Blog post not found at path: app/blog/[slug]/layout.tsx, slug: ${params.slug}`)
    notFound()
  }

  return <article>{children}</article>
}
