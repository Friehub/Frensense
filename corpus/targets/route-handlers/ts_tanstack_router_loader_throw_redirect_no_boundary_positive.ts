// [frensense]
// observation: a route loader throws a redirect (e.g., `throw redirect({ to: '/login' })`) but there is no `onError` handler on the route, so the error boundary catches it and potentially re-renders the loader, creating an infinite loop
// impact: infinite redirect loop — the loader throws a redirect, the error boundary re-renders, the loader throws again, and the browser tab hangs or crashes
// improvement: define an `onError` handler on the route that handles the redirect, or use `loader` to return a redirect response instead of throwing
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium
// runtime_probe: redirect

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
  component: () => <div>Dashboard</div>,
})

const routeTree = rootRoute.addChildren([protectedRoute])

const router = createRouter({ routeTree })
