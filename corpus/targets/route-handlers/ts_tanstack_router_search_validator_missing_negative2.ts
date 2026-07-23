// SAFE: Manual validation of search params in the loader

import { createRoute, createRootRoute, createRouter } from '@tanstack/react-router'
import { z } from 'zod'

const itemsSearchSchema = z.object({
  page: z.number().positive().default(1),
  sort: z.enum(['name', 'date', 'price']).default('date'),
})

const rootRoute = createRootRoute()

const itemsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/items',
  loader: async ({ location }) => {
    const search = itemsSearchSchema.parse(location.search)
    return search
  },
  component: () => <div>Items</div>,
})

const routeTree = rootRoute.addChildren([itemsRoute])

const router = createRouter({ routeTree })
