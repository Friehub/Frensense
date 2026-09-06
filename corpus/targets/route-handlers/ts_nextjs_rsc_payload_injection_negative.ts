// SAFE: RSC payload is validated against an allowlist of known component types before dynamic import
// CVE: CVE-2025-66478

import { cache } from 'react'

const ALLOWED_COMPONENTS = new Set(['WeatherWidget', 'StockTicker', 'CalendarWidget'] as const)

type AllowedComponent = typeof ALLOWED_COMPONENTS extends Set<infer T> ? T : never

interface RscPayload {
  component: AllowedComponent
  props: Record<string, unknown>
}

async function fetchRscPayload(url: string): Promise<unknown> {
  const res = await fetch(url)
  return res.json()
}

function validatePayload(raw: unknown): RscPayload {
  if (typeof raw !== 'object' || raw === null) throw new Error('invalid payload')
  const { component, props } = raw as Record<string, unknown>
  if (typeof component !== 'string' || !ALLOWED_COMPONENTS.has(component as AllowedComponent)) {
    throw new Error('disallowed component type')
  }
  if (typeof props !== 'object' || props === null) throw new Error('invalid props')
  return { component: component as AllowedComponent, props: props as Record<string, unknown> }
}

export default async function DynamicWidget({ dataUrl }: { dataUrl: string }) {
  const raw = await fetchRscPayload(dataUrl)
  const payload = validatePayload(raw)

  const Component = (await import(`@/components/${payload.component}`)).default

  return <Component {...payload.props} />
}
