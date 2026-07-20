// SAFE: only non-sensitive data passed via provide/inject; token stored in composable with controlled access

import { provide, inject, ref, type Ref } from 'vue'
import { useAuthStore } from './stores/auth'

const USER_NAME_KEY = Symbol('userName')

export function useUserProvider(name: Ref<string>) {
  provide(USER_NAME_KEY, name)
}

export function useUserName(): Ref<string> {
  const name = inject(USER_NAME_KEY)
  if (!name) throw new Error('no user name provided')
  return name
}

export function useAuthToken(): string | null {
  const store = useAuthStore()
  return store.token
}
