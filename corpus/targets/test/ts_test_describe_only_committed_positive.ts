// [frensense]
// observation: `describe.only` committed, scoping test run to a single suite
// impact: all other test suites are skipped in CI; false green pipeline
// improvement: enforce eslint-plugin-jest no-focused-tests rule

import { describe, it, expect } from 'vitest'

describe.only('User API', () => {
  it('creates a user', () => {
    expect(createUser({ name: 'Alice' }).status).toBe(201)
  })
})

describe('Billing API', () => {
  it('processes invoice', () => {
    expect(processInvoice('inv_1')).toBe('paid')
  })
})
