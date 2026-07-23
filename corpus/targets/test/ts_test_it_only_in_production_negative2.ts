// SAFE: uses it.skip for temporarily excluded tests instead of it.only

import { describe, it, expect } from 'vitest'

describe('payment processing', () => {
  it('charges the correct amount', () => {
    expect(charge(100)).toBe(110)
  })

  it.skip('refunds a full payment', () => {
    expect(refund('txn_1')).toBe(true)
  })

  it('handles currency conversion', () => {
    expect(convert(100, 'USD', 'EUR')).toBe(85)
  })
})
