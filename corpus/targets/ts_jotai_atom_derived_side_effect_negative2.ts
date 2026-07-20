// SAFE: uses jotai-effect for scoped side effects

import { atom } from 'jotai'
import { atomEffect } from 'jotai-effect'

export const userIdAtom = atom<string>('')
export const auditEffect = atomEffect((get) => {
  const uid = get(userIdAtom)
  fetch('/api/audit', { method: 'POST', body: JSON.stringify({ uid }) })
})
