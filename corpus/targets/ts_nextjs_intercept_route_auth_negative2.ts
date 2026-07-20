// SAFE: uses a HOC pattern to enforce auth on all intercepted routes

'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useSession } from '@/lib/auth'

function withInterceptGuard<P extends object>(Component: React.ComponentType<P>) {
  return function Guarded(props: P) {
    const { data: session, status } = useSession()
    const router = useRouter()
    useEffect(() => {
      if (status === 'unauthenticated') router.push('/login')
    }, [status, router])
    if (status !== 'authenticated') return null
    return <Component {...props} />
  }
}

function PhotoPage({ params }: { params: { id: string } }) {
  return <div>Photo {params.id}</div>
}

export default withInterceptGuard(PhotoPage)
