// SAFE: onError handler on the route catches the redirect and handles it gracefully

import { createRouter, createRoute, createRootRoute, redirect } from '@tanstack/react-router'

const rootRoute = createRootRoute()

const protectedRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/dashboard',
  loader: async () => {
    const token = localStorage.getItem('auth_token')
    if (!token) {
      throw redirect({ to: '/login' })
    }
    const res = await fetch('/api/dashboard', {
      headers: { Authorization: `Bearer ${token}` },
    })
    return res.json()
  },
  onError: (error) => {
    if ((error as any)?.code === 'REDIRECT') {
      return
    }
    console.error('Loader error:', error)
  },
  component: () => <div>Dashboard</div>,
})

const routeTree = rootRoute.addChildren([protectedRoute])

const router = createRouter({ routeTree })
