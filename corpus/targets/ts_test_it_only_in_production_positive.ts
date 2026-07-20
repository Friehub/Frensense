// [frensense]
// observation: `it.only` or `test.only` committed to the main branch, causing all other tests to be skipped
// impact: CI passes despite other tests failing; regressions go undetected
// improvement: use lint rules (jest/no-focused-tests) to block .only in CI

import { describe, it, expect } from 'vitest'

describe('payment processing', () => {
  it.only('charges the correct amount', () => {
    expect(charge(100)).toBe(110)
  })

  it('refunds a full payment', () => {
    expect(refund('txn_1')).toBe(true)
  })

  it('handles currency conversion', () => {
    expect(convert(100, 'USD', 'EUR')).toBe(85)
  })
})
