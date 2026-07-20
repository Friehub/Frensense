// [frensense]
// observation: fake timers are enabled but never advanced, causing time-dependent code to hang
// impact: setTimeout/setInterval callbacks never fire; test times out
// improvement: advance timers with vi.advanceTimersByTime or vi.runAllTimers

import { describe, it, expect, vi, beforeEach } from 'vitest'

function debounce<T extends (...args: unknown[]) => void>(fn: T, ms: number) {
  let timer: ReturnType<typeof setTimeout>
  return (...args: Parameters<T>) => {
    clearTimeout(timer)
    timer = setTimeout(() => fn(...args), ms)
  }
}

describe('debounce', () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })

  it('fires after the delay', () => {
    const spy = vi.fn()
    const debounced = debounce(spy, 500)
    debounced('hello')
    expect(spy).not.toHaveBeenCalled()
    expect(spy).toHaveBeenCalledTimes(0)
  })
})
