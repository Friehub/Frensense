// [frensense]
// observation: Astro client:load or client:visible directive value is constructed from user input
// impact: attacker can control which component is loaded or inject arbitrary client-side scripts
// improvement: never interpolate user input into component names; use a whitelist map

import type { APIRoute } from 'astro'

interface WidgetConfig {
  component: string
  props: Record<string, unknown>
}

export const prerender = false

export const GET: APIRoute = async ({ url }) => {
  const widgetName = url.searchParams.get('widget') ?? 'default'
  const config: WidgetConfig = {
    component: `/src/components/widgets/${widgetName}.astro`,
    props: { theme: url.searchParams.get('theme') }
  }

  return new Response(JSON.stringify(config), {
    headers: { 'Content-Type': 'application/json' }
  })
}
