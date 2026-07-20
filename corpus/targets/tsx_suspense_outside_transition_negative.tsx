// SAFE: page state update is wrapped in startTransition, so React can avoid showing the fallback for fast loads

'use client'

import { Suspense, startTransition, useState } from 'react'

export default function App() {
  const [page, setPage] = useState<'home' | 'settings'>('home')

  function navigate(next: 'home' | 'settings') {
    startTransition(() => {
      setPage(next)
    })
  }

  return (
    <div>
      <button onClick={() => navigate('settings')}>Settings</button>
      <button onClick={() => navigate('home')}>Home</button>
      <Suspense fallback={<div>Loading...</div>}>
        {page === 'home' ? <HomePage /> : <SettingsPage />}
      </Suspense>
    </div>
  )
}

async function HomePage() {
  await new Promise((r) => setTimeout(r, 100))
  return <div>Home</div>
}

async function SettingsPage() {
  await new Promise((r) => setTimeout(r, 100))
  return <div>Settings</div>
}
