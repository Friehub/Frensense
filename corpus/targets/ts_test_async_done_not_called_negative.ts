// SAFE: calls done() in the callback to signal test completion

import { describe, it, expect } from 'vitest'

function fetchData(cb: (data: string) => void) {
  setTimeout(() => cb('data-loaded'), 100)
}

describe('fetchData', () => {
  it('loads data', (done) => {
    fetchData((data) => {
      expect(data).toBe('data-loaded')
      done()
    })
  })
})
