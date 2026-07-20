// SAFE: The layout does not fetch user data; user-specific content is deferred to a client component that hydrates from a cookie or token

import { ReactNode } from 'react'
import ClientUserBar from '@/components/ClientUserBar'

export default function AppLayout({ children }: { children: ReactNode }) {
  return (
    <html>
      <body>
        <header>
          <ClientUserBar />
        </header>
        {children}
      </body>
    </html>
  )
}
