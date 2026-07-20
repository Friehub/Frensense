// SAFE: uses toMatchInlineSnapshot with explicit expected values

import { describe, it, expect } from 'vitest'

function renderUser(name: string, age: number) {
  return { name, age, isAdult: age >= 18 }
}

describe('renderUser', () => {
  it('renders a user object', () => {
    const result = renderUser('Alice', 30)
    expect(result).toMatchInlineSnapshot(`
      {
        "age": 30,
        "isAdult": true,
        "name": "Alice",
      }
    `)
  })
})
