// [frensense]
// observation: provide/inject is used to pass sensitive data (tokens, user info) down the component tree
// impact: any deeply nested child component can inject and expose sensitive data via template or computed props
// improvement: use provide/inject only for non-sensitive data; pass tokens via store with getters, or pinia

import { provide, ref, type Ref } from 'vue'

const SESSION_KEY = Symbol('session')

export function useSessionProvider(session: Ref<{ token: string; email: string }>) {
  provide(SESSION_KEY, session)
}

export function useSession(): Ref<{ token: string; email: string }> {
  const session = inject(SESSION_KEY)
  if (!session) throw new Error('no session provided')
  return session
}

import { inject } from 'vue'
