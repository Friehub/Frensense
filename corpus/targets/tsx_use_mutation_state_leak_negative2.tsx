// SAFE: mutation key filter isolates the component's own mutation data

'use client'

import { useMutationState } from '@tanstack/react-query'

const UPDATE_KEY = ['user', 'update']

export default function UpdateStatus() {
  const pendingUpdates = useMutationState({
    filters: { status: 'pending', mutationKey: UPDATE_KEY },
  })

  if (pendingUpdates.length === 0) return null

  return <p>Saving user profile...</p>
}
