// SAFE: beforeLoad checks authentication before allowing access to the route

import { createRouter, createRoute, createRootRoute, redirect } from '@tanstack/react-router'

const rootRoute = createRootRoute()

const profileRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/profile/$userId',
  beforeLoad: () => {
    const token = localStorage.getItem('auth_token')
    if (!token) {
      throw redirect({ to: '/login' })
    }
  },
  loader: async ({ params }) => {
    const token = localStorage.getItem('auth_token')
    const res = await fetch(`/api/users/${params.userId}/settings`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })
    return res.json()
  },
  component: () => <div>Profile Settings</div>,
})

const routeTree = rootRoute.addChildren([profileRoute])

const router = createRouter({ routeTree })
