// SAFE: route loader defines a typed DTO and transforms the API response before returning

import { Route } from '@tanstack/react-router'

interface RawUser {
  id: string
  name: string
  email: string
  ssn: string
  createdAt: string
}

interface UserProfileDTO {
  id: string
  name: string
  email: string
  memberSince: string
}

const userRoute = new Route({
  path: '/users/$userId',
  validateSearch: (search: Record<string, unknown>) => ({
    tab: (search.tab as string) ?? 'overview',
  }),
  loader: async ({ params }): Promise<UserProfileDTO> => {
    const res = await fetch(`/api/users/${params.userId}`)
    const data: RawUser = await res.json()
    return {
      id: data.id,
      name: data.name,
      email: data.email,
      memberSince: data.createdAt,
    }
  },
  component: ({ useLoader }) => {
    const data = useLoader()
    return <div>{data.name} — {data.email} (since {data.memberSince})</div>
  },
})
