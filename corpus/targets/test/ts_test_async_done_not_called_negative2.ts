// SAFE: returns a promise instead of using the done callback

import { describe, it, expect } from 'vitest'

function fetchData(): Promise<string> {
  return new Promise((resolve) => setTimeout(() => resolve('data-loaded'), 100))
}

describe('fetchData', () => {
  it('loads data', async () => {
    const data = await fetchData()
    expect(data).toBe('data-loaded')
  })
})
