// SAFE: mocks are scoped within the test using mockImplementationOnce

import { describe, it, expect, vi } from 'vitest'
import * as api from './api'

describe('UserService', () => {
  it('fetches user', async () => {
    const mock = vi.spyOn(api, 'getUser').mockResolvedValue({ id: 1, name: 'Mocked' })
    const user = await api.getUser(1)
    expect(user.name).toBe('Mocked')
    mock.mockRestore()
  })

  it('fetches another user', async () => {
    const user = await api.getUser(2)
    expect(user.name).toBe('Alice')
  })
})
