// SAFE: each mutation uses a unique domain-prefixed mutation key, preventing cross-component interference

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
