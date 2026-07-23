// SAFE: restores the mock in afterEach to prevent cross-test pollution

import { describe, it, expect, vi, afterEach } from 'vitest'
import * as api from './api'

afterEach(() => {
  vi.restoreAllMocks()
})

describe('UserService', () => {
  it('fetches user', async () => {
    vi.spyOn(api, 'getUser').mockResolvedValue({ id: 1, name: 'Mocked' })
    const user = await api.getUser(1)
    expect(user.name).toBe('Mocked')
  })

  it('fetches another user', async () => {
    const user = await api.getUser(2)
    expect(user.name).toBe('Alice')
  })
})
