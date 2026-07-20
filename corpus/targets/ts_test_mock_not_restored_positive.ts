// [frensense]
// observation: jest.spyOn / vi.spyOn used without mock restoration after the test
// impact: mocked functions leak into subsequent tests, causing cross-test pollution
// improvement: always call mockRestore() in afterEach or use restoreMocks:true config

import { describe, it, expect, vi } from 'vitest'
import * as api from './api'

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
