// SAFE: side effect is triggered imperatively via a write atom, not in a derived read

import { atom } from 'jotai'

export const userIdAtom = atom<string>('')
export const auditLogAtom = atom(null, async (_get, set, uid: string) => {
  await fetch('/api/audit', { method: 'POST', body: JSON.stringify({ uid }) })
  set(userIdAtom, uid)
})
