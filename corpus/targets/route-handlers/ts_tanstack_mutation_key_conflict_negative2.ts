// SAFE: mutation keys include domain-specific prefixes that prevent cross-mutation state interference

import { useMutation, useMutationState } from '@tanstack/react-query'

export function useUpdateProfile() {
  return useMutation({
    mutationKey: ['profile', 'update'],
    mutationFn: async (data: { name: string }) => {
      const res = await fetch('/api/profile', {
        method: 'PUT',
        body: JSON.stringify(data),
      })
      return res.json()
    },
  })
}

export function useUpdatePassword() {
  return useMutation({
    mutationKey: ['password', 'update'],
    mutationFn: async (data: { password: string }) => {
      const res = await fetch('/api/password', {
        method: 'PUT',
        body: JSON.stringify(data),
      })
      return res.json()
    },
  })
}

export function ProfileUpdateListener() {
  const mutations = useMutationState({
    filters: { mutationKey: ['profile', 'update'], status: 'success' },
  })

  return <p>Profile updated: {mutations.length} times</p>
}
