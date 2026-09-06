// SAFE: uses `useTransition` hook to get `isPending` status and show a subtle loading indicator instead of the full fallback

'use client'

import { Suspense, useTransition, useState } from 'react'

export default function App() {
  const [page, setPage] = useState<'home' | 'settings'>('home')
  const [isPending, startTransition] = useTransition()

  function navigate(next: 'home' | 'settings') {
    startTransition(() => {
      setPage(next)
    })
  }

  return (
    <div>
      {isPending && <div style={{ opacity: 0.5 }}>Navigating...</div>}
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
