// SAFE: adds explicit property assertions alongside the snapshot

import { describe, it, expect } from 'vitest'

function renderUser(name: string, age: number) {
  return { name, age, isAdult: age >= 18 }
}

describe('renderUser', () => {
  it('renders a user object', () => {
    const result = renderUser('Alice', 30)
    expect(result.name).toBe('Alice')
    expect(result.age).toBe(30)
    expect(result.isAdult).toBe(true)
    expect(result).toMatchSnapshot()
  })
})
