// SAFE: useQuery is used instead of useSuspenseQuery, so loading state is handled without Suspense

import { useQuery } from '@tanstack/react-query'

interface User {
  id: string
  name: string
}

export function UserProfile({ userId }: { userId: string }) {
  const { data, isLoading, error } = useQuery<User>({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })

  if (isLoading) return <div>Loading...</div>
  if (error) return <div>Error loading user</div>

  return <div>{data?.name}</div>
}

export default function App() {
  return <UserProfile userId="123" />
}
