// SAFE: Suspense boundary wraps the component using useSuspenseQuery

import { Suspense } from 'react'
import { useSuspenseQuery } from '@tanstack/react-query'

interface User {
  id: string
  name: string
}

function UserProfile({ userId }: { userId: string }) {
  const { data } = useSuspenseQuery<User>({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })

  return <div>{data.name}</div>
}

export default function App() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <UserProfile userId="123" />
    </Suspense>
  )
}
