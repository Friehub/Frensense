// [frensense]
// observation: RSC payload parsing dynamically imports arbitrary components from the payload's component field without validation or allowlist checking, enabling injection of any server component.
// impact: An attacker can craft a malicious RSC payload referencing internal framework components like `__error_renderer` or `__redirect` to achieve remote code execution (CVSS 10.0, CVE-2025-66478).
// improvement: Validate the component name against an allowlist of known safe component types before dynamic import.
// CVE: CVE-2025-66478

import { cache } from 'react'

interface RscPayload {
  component: string
  props: Record<string, unknown>
}

async function fetchRscPayload(url: string): Promise<unknown> {
  const res = await fetch(url)
  return res.json()
}

export default async function DynamicWidget({ dataUrl }: { dataUrl: string }) {
  const raw = await fetchRscPayload(dataUrl) as RscPayload
  const Component = (await import(`@/components/${raw.component}`)).default
  return <Component {...raw.props} />
}
