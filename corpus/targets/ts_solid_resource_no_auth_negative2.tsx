// SAFE: credentials included and error handling added for unauthorized responses

import { Component, createResource } from 'solid-js'

interface User {
  id: number
  name: string
}

const UserProfile: Component<{ userId: number }> = (props) => {
  const [user] = createResource(() => props.userId, async (id, { refetching }) => {
    const res = await fetch(`/api/users/${id}`, {
      credentials: 'include',
      headers: {
        'X-CSRF-Token': document.querySelector('meta[name=csrf]')?.getAttribute('content') ?? ''
      }
    })
    if (res.status === 401) {
      window.location.href = '/login'
      return null
    }
    return res.json()
  })

  return (
    <div>
      <h1>{user()?.name}</h1>
    </div>
  )
}

export default UserProfile
