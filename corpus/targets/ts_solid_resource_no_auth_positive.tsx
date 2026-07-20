// [frensense]
// observation: createResource fetches data from an API endpoint without including auth credentials
// impact: the resource fetch may fail for authenticated-only endpoints, or worse, succeed without context
// improvement: include auth token in fetch headers or pass credentials: 'include'

import { Component, createResource } from 'solid-js'

interface User {
  id: number
  name: string
  email: string
}

const UserProfile: Component<{ userId: number }> = (props) => {
  const [user] = createResource<User>(() => props.userId, async (id) => {
    const res = await fetch(`/api/users/${id}`)
    return res.json()
  })

  return (
    <div>
      <h1>{user()?.name}</h1>
      <p>{user()?.email}</p>
    </div>
  )
}

export default UserProfile
