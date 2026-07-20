// [frensense]
// observation: `<Profiler>` `id` prop is set directly from user input, allowing DOM injection or XSS when the id is rendered into the DOM
// impact: cross-site scripting (XSS) — malicious `id` values like `"><img src=x onerror=alert(1)>` can be injected into the DOM
// improvement: validate or sanitize the id prop, or use a fixed allowlist of profiler IDs

'use client'

import { Profiler, useState } from 'react'
import type { ProfilerOnRender } from 'react'

function onRenderCallback: ProfilerOnRender = (id, phase, actualDuration) => {
  console.log({ id, phase, actualDuration })
}

export default function ProfilerPage() {
  const [sectionId, setSectionId] = useState('main')

  return (
    <div>
      <input value={sectionId} onChange={(e) => setSectionId(e.target.value)} />
      <Profiler id={sectionId} onRender={onRenderCallback}>
        <p>Content to profile</p>
      </Profiler>
    </div>
  )
}
