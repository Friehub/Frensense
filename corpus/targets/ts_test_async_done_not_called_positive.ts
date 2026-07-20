// [frensense]
// observation: async test uses the `done` callback but never calls it on success
// impact: test times out even when the operation succeeds; CI runs slowly or hangs
// improvement: always call `done()` in both success and error paths, or return a promise

import { describe, it, expect } from 'vitest'

function fetchData(cb: (data: string) => void) {
  setTimeout(() => cb('data-loaded'), 100)
}

describe('fetchData', () => {
  it('loads data', (done) => {
    fetchData((data) => {
      expect(data).toBe('data-loaded')
    })
  })
})
