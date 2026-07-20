// SAFE: User-specific data is wrapped in a Suspense boundary so it is excluded from the static PPR shell and rendered dynamically per-request

import { Suspense, ReactNode } from 'react'

async function UserAvatar() {
  const { cookies } = await import('next/headers')
  const user = await fetch(`https://api.example.com/me`, {
    headers: { cookie: (await cookies()).toString() },
  }).then(r => r.json()) as { name: string; avatar: string }

  return (
    <>
      <img src={user.avatar} alt={user.name} />
      <span>{user.name}</span>
    </>
  )
}

export default function AppLayout({ children }: { children: ReactNode }) {
  return (
    <html>
      <body>
        <header>
          <Suspense fallback={<span>Loading...</span>}>
            <UserAvatar />
          </Suspense>
        </header>
        {children}
      </body>
    </html>
  )
}
