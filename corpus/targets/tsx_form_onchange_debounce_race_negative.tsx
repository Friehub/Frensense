// SAFE: uses useRef to always send the latest value

'use client'

import { useState, useRef, useCallback } from 'react'

export default function SearchPage() {
  const [query, setQuery] = useState('')
  const latestRef = useRef(query)
  const timerRef = useRef<ReturnType<typeof setTimeout>>()

  const handleChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value
    setQuery(value)
    latestRef.current = value
    clearTimeout(timerRef.current)
    timerRef.current = setTimeout(() => {
      fetch(`/api/search?q=${latestRef.current}`)
    }, 300)
  }, [])

  return <input value={query} onChange={handleChange} />
}
