// SAFE: the route loader transforms the API response to only return fields needed by the component

import { Route } from '@tanstack/react-router'

interface UserData {
  id: string
  name: string
  email: string
  ssn: string
  internalNotes: string
}

interface PublicUserData {
  id: string
  name: string
  email: string
}

const userRoute = new Route({
  path: '/users/$userId',
  loader: async ({ params }): Promise<PublicUserData> => {
    const res = await fetch(`/api/users/${params.userId}`)
    const data: UserData = await res.json()
    return {
      id: data.id,
      name: data.name,
      email: data.email,
    }
  },
  component: ({ useLoader }) => {
    const data = useLoader()
    return <div>{data.name} — {data.email}</div>
  },
})
