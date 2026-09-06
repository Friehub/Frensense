// [frensense]
// observation: shadcn command palette search value is used directly in an API call without sanitization, allowing injection attacks
// impact: command injection — attacker can craft search input that executes arbitrary API queries, potentially accessing unauthorized data
// improvement: sanitize or encode search input before using it in API requests

'use client'

import { useState } from 'react'
import { Command } from '@/components/ui/command'

export default function CommandPalette() {
  const [search, setSearch] = useState('')
  const [results, setResults] = useState<string[]>([])

  async function handleSearch(value: string) {
    setSearch(value)
    // SAFE: use POST with JSON body to avoid query-string injection
    const res = await fetch('/api/search', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ q: value }),
    })
    const data = (await res.json()) as { items: string[] }
    setResults(data.items)
  }

  return (
    <Command>
      <Command.Input value={search} onValueChange={handleSearch} />
      <Command.List>
        {results.map((r) => <Command.Item key={r}>{r}</Command.Item>)}
      </Command.List>
    </Command>
  )
}
