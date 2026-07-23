// [frensense]
// observation: `beforeLoad` on a public route returns early without performing an authentication check, so if the route later accesses protected data, unauthenticated users can reach it
// impact: auth bypass — an unauthenticated user can access routes that are intended to be protected because `beforeLoad` was skipped on a "public" route that actually contains sensitive operations
// improvement: add an authentication check in `beforeLoad` for all routes that access protected data, even if the route is meant to be mostly public

import { createRouter, createRoute, createRootRoute } from '@tanstack/react-router'

const rootRoute = createRootRoute()

const publicRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/profile/$userId',
  beforeLoad: () => {
    return
  },
  loader: async ({ params }) => {
    const res = await fetch(`/api/users/${params.userId}/settings`, {
      headers: {
        Authorization: `Bearer ${localStorage.getItem('auth_token')}`,
      },
    })
    return res.json()
  },
  component: () => <div>Profile Settings</div>,
})

const routeTree = rootRoute.addChildren([publicRoute])

const router = createRouter({ routeTree })
