// SAFE: validateSearch uses zod to enforce the expected search param schema

import { createRoute, createRootRoute, createRouter } from '@tanstack/react-router'
import { z } from 'zod'

const itemsSearchSchema = z.object({
  page: z.number().positive().default(1),
  sort: z.enum(['name', 'date', 'price']).default('date'),
  filter: z.string().optional(),
})

const rootRoute = createRootRoute()

const itemsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/items',
  validateSearch: itemsSearchSchema,
  component: () => <div>Items</div>,
})

const routeTree = rootRoute.addChildren([itemsRoute])

const router = createRouter({ routeTree })
