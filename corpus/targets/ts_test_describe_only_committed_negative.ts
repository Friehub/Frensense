// SAFE: removes .only so all suites execute

import { describe, it, expect } from 'vitest'

describe('User API', () => {
  it('creates a user', () => {
    expect(createUser({ name: 'Alice' }).status).toBe(201)
  })
})

describe('Billing API', () => {
  it('processes invoice', () => {
    expect(processInvoice('inv_1')).toBe('paid')
  })
})
