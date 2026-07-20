// SAFE: component names validated against a regex pattern before use

import type { APIRoute } from 'astro'

const VALID_COMPONENT = /^[a-zA-Z]+$/

export const prerender = false

export const GET: APIRoute = async ({ url }) => {
  const raw = url.searchParams.get('widget') ?? 'stats'
  const widgetName = VALID_COMPONENT.test(raw) ? raw : 'stats'

  return new Response(JSON.stringify({
    component: `/src/components/widgets/${widgetName}.astro`
  }), {
    headers: { 'Content-Type': 'application/json' }
  })
}
