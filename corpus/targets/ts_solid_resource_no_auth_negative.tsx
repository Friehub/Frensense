// SAFE: auth token included in fetch headers within createResource

import { Component, createResource } from 'solid-js'

interface User {
  id: number
  name: string
}

const UserProfile: Component<{ userId: number }> = (props) => {
  const [user] = createResource(() => props.userId, async (id) => {
    const token = localStorage.getItem('auth_token')
    const res = await fetch(`/api/users/${id}`, {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
    if (!res.ok) throw new Error('Failed to fetch')
    return res.json()
  })

  return (
    <div>
      <h1>{user()?.name}</h1>
    </div>
  )
}

export default UserProfile
