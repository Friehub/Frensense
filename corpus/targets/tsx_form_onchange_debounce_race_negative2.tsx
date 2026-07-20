// SAFE: uses AbortController to cancel stale requests

'use client'

import { useState, useRef } from 'react'

export default function SearchPage() {
  const [query, setQuery] = useState('')
  const controllerRef = useRef<AbortController>()

  async function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value
    setQuery(value)
    controllerRef.current?.abort()
    const controller = new AbortController()
    controllerRef.current = controller
    fetch(`/api/search?q=${value}`, { signal: controller.signal })
  }

  return <input value={query} onChange={handleChange} />
}
