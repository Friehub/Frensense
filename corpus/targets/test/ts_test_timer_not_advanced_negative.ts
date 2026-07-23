// SAFE: advances fake timers to trigger the pending setTimeout

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
    vi.advanceTimersByTime(500)
    expect(spy).toHaveBeenCalledTimes(1)
    expect(spy).toHaveBeenCalledWith('hello')
  })
})
