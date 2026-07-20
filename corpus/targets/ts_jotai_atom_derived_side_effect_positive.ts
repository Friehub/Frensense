// [frensense]
// observation: a Jotai derived atom performs side effects (writing to a file, making HTTP calls)
// impact: side effects fire on every read, potentially multiple times per render, causing data corruption
// improvement: use `atomEffect` from `jotai-effect` or split into a separate action

import { atom } from 'jotai'

export const userIdAtom = atom<string>('')
export const auditLogAtom = atom(async (get) => {
  const uid = get(userIdAtom)
  await fetch('/api/audit', { method: 'POST', body: JSON.stringify({ uid }) })
  return uid
})
