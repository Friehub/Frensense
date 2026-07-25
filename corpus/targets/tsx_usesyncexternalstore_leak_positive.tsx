// [frensense]
// observation: `useSyncExternalStore` subscribes to a store whose snapshot is a mutable object returned directly, causing consumers to see tearing (inconsistent values) during concurrent rendering
// impact: UI displays inconsistent state — some components see old values while others see new values during the same render
// improvement: return an immutable snapshot (e.g., spread or structuredClone) from the getSnapshot function
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use client'

import { useSyncExternalStore } from 'react'

interface Store {
  count: number
  label: string
}

const store = { count: 0, label: 'initial' }

const listeners = new Set<() => void>()

export function setStore(value: Partial<Store>) {
  Object.assign(store, value)
  listeners.forEach((l) => l())
}

function subscribe(cb: () => void) {
  listeners.add(cb)
  return () => listeners.delete(cb)
}

function getSnapshot(): Store {
  return store
}

export default function Counter() {
  const snapshot = useSyncExternalStore(subscribe, getSnapshot)

  return (
    <div>
      <p>Count: {snapshot.count}</p>
      <p>Label: {snapshot.label}</p>
      <button onClick={() => setStore({ count: snapshot.count + 1 })}>Increment</button>
    </div>
  )
}
