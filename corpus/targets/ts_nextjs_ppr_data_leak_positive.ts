// [frensense]
// observation: A layout uses Partial Prerendering (PPR) with a static shell that contains user-specific data from the first request, causing that user's data to be cached and served to subsequent visitors.
// impact: User A's personal information (name, avatar) rendered in the PPR shell is served to User B and all anonymous visitors via the shared static response.
// improvement: Ensure user-specific data is excluded from the static PPR shell and only rendered in the dynamic suspended boundaries.

import { ReactNode } from 'react'

export default async function AppLayout({ children }: { children: ReactNode }) {
  const { cookies } = await import('next/headers')
  const user = await fetch(`https://api.example.com/me`, {
    headers: { cookie: (await cookies()).toString() },
  }).then(r => r.json()) as { name: string; avatar: string }

  return (
    <html>
      <body>
        <header>
          <img src={user.avatar} alt={user.name} />
          <span>{user.name}</span>
        </header>
        {children}
      </body>
    </html>
  )
}
