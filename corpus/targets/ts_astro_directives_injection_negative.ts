// SAFE: component names are mapped via a whitelist, never interpolated from user input

import type { APIRoute } from 'astro'

const WIDGET_MAP: Record<string, string> = {
  chart: '/src/components/widgets/Chart.astro',
  table: '/src/components/widgets/DataTable.astro',
  stats: '/src/components/widgets/Stats.astro'
}

export const prerender = false

export const GET: APIRoute = async ({ url }) => {
  const widgetKey = url.searchParams.get('widget') ?? 'stats'
  const component = WIDGET_MAP[widgetKey] ?? WIDGET_MAP.stats

  return new Response(JSON.stringify({ component }), {
    headers: { 'Content-Type': 'application/json' }
  })
}
