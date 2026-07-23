// [frensense]
// observation: snapshot test calls `toMatchSnapshot()` without any explicit assertion
// impact: test passes automatically on first run without verifying correctness; stale snapshots hide regressions
// improvement: always review snapshot diffs; use inline snapshots for small outputs

import { describe, it, expect } from 'vitest'

function renderUser(name: string, age: number) {
  return { name, age, isAdult: age >= 18 }
}

describe('renderUser', () => {
  it('renders a user object', () => {
    const result = renderUser('Alice', 30)
    expect(result).toMatchSnapshot()
  })
})
