// [frensense]
// observation: A Server Component receives and renders an RSC payload object without validating its component tree structure, allowing an attacker-controlled component to be instantiated.
// impact: Remote code execution via crafted RSC payload — attacker can inject arbitrary server components that execute in the privileged RSC context (CVE-2025-66478, CVSS 10.0).
// improvement: Validate the RSC payload against a schema that only allows known, safe component types before rendering.
// CVE: CVE-2025-66478

import { cache } from 'react'

interface RscPayload {
  component: string
  props: Record<string, unknown>
}

async function fetchRscPayload(url: string): Promise<RscPayload> {
  const res = await fetch(url)
  return res.json() as Promise<RscPayload>
}

export default async function DynamicWidget({ dataUrl }: { dataUrl: string }) {
  const payload = await fetchRscPayload(dataUrl)

  const Component = (await import(`@/components/${payload.component}`)).default

  return <Component {...payload.props} />
}
