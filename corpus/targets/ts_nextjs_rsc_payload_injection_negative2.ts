// SAFE: RSC payload uses a tagged union with a discriminant resolved via a switch statement rather than dynamic import
// CVE: CVE-2025-66478

import { cache } from 'react'
import WeatherWidget from '@/components/WeatherWidget'
import StockTicker from '@/components/StockTicker'
import CalendarWidget from '@/components/CalendarWidget'

type ComponentType = 'WeatherWidget' | 'StockTicker' | 'CalendarWidget'

interface RscPayload {
  component: ComponentType
  props: Record<string, unknown>
}

async function fetchRscPayload(url: string): Promise<unknown> {
  const res = await fetch(url)
  return res.json()
}

function validatePayload(raw: unknown): RscPayload {
  if (typeof raw !== 'object' || raw === null) throw new Error('invalid payload')
  const { component, props } = raw as Record<string, unknown>
  if (component !== 'WeatherWidget' && component !== 'StockTicker' && component !== 'CalendarWidget') {
    throw new Error('unknown component type')
  }
  if (typeof props !== 'object' || props === null) throw new Error('invalid props')
  return { component, props: props as Record<string, unknown> }
}

function renderComponent(payload: RscPayload) {
  switch (payload.component) {
    case 'WeatherWidget':
      return <WeatherWidget {...payload.props} />
    case 'StockTicker':
      return <StockTicker {...payload.props} />
    case 'CalendarWidget':
      return <CalendarWidget {...payload.props} />
  }
}

export default async function DynamicWidget({ dataUrl }: { dataUrl: string }) {
  const raw = await fetchRscPayload(dataUrl)
  const payload = validatePayload(raw)
  return renderComponent(payload)
}
