// SAFE: The loader returns a redirect response instead of throwing

import { createRouter, createRoute, createRootRoute, redirect } from '@tanstack/react-router'

const rootRoute = createRootRoute()

const protectedRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/dashboard',
  loader: async () => {
    const token = localStorage.getItem('auth_token')
    if (!token) {
      return redirect({ to: '/login' })
    }
    const res = await fetch('/api/dashboard', {
      headers: { Authorization: `Bearer ${token}` },
    })
    return res.json()
  },
  component: () => <div>Dashboard</div>,
})

const routeTree = rootRoute.addChildren([protectedRoute])

const router = createRouter({ routeTree })
