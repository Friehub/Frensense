// [frensense]
// observation: Suspense is triggered by a state update without being wrapped in `startTransition`, causing the fallback to appear on every navigation
// impact: unwanted fallbacks flash on every page transition, creating a jarring UX with loading spinners for fast navigations
// improvement: wrap the state update in `startTransition` to let React avoid showing the fallback if the data loads quickly

'use client'

import { Suspense, useState } from 'react'

export default function App() {
  const [page, setPage] = useState<'home' | 'settings'>('home')

  return (
    <div>
      <button onClick={() => setPage('settings')}>Settings</button>
      <button onClick={() => setPage('home')}>Home</button>
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
