// [frensense]
// observation: React portal renders outside the app's root DOM element — content appears in a detached or unexpected container
// impact: CSS scoping breaks, event bubbling misses parent handlers, and content may be targetable by unrelated CSS selectors (information leak via styling)
// improvement: ensure the portal target is a child of the app root or use a contained portal approach
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium

'use client'

import { createPortal } from 'react-dom'

export default function Modal() {
  const target = document.getElementById('outside-root')
  if (!target) return null
  return createPortal(
    <div className="modal">
      <p>Sensitive user data</p>
    </div>,
    target
  )
}
