// [frensense]
// observation: a route definition does not include a `validateSearch` option, so search params are not validated against a schema and arbitrary values can be injected
// impact: search param injection — an attacker can craft a URL with malicious or unexpected search param values that bypass intended constraints (e.g., `?role=admin&page=-1`), potentially causing injection attacks or logic errors
// improvement: add a `validateSearch` option to the route definition that uses zod (or another validator) to define and enforce the expected search param schema

import { createRoute, createRootRoute, createRouter } from '@tanstack/react-router'

const rootRoute = createRootRoute()

const itemsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/items',
  component: () => <div>Items</div>,
})

const routeTree = rootRoute.addChildren([itemsRoute])

const router = createRouter({ routeTree })
