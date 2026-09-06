// SAFE: Component map lookup instead of dynamic string interpolation; only pre-registered components are renderable
// CVE: CVE-2025-66478

const componentRegistry = {
  weather: () => import('@/components/WeatherWidget'),
  stock: () => import('@/components/StockTicker'),
  calendar: () => import('@/components/CalendarWidget'),
} as const

type RegistryKey = keyof typeof componentRegistry

export default async function SafeDynamicWidget({ kind }: { kind: string }) {
  if (!(kind in componentRegistry)) {
    throw new Error(`Unknown component: ${kind}`)
  }
  const loader = componentRegistry[kind as RegistryKey]
  const Component = (await loader()).default
  return <Component />
}
