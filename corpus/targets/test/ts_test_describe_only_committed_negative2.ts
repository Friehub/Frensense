// SAFE: uses describe.skip for temporary exclusion instead of describe.only

import { describe, it, expect } from 'vitest'

describe('User API', () => {
  it('creates a user', () => {
    expect(createUser({ name: 'Alice' }).status).toBe(201)
  })
})

describe.skip('Billing API', () => {
  it('processes invoice', () => {
    expect(processInvoice('inv_1')).toBe('paid')
  })
})
