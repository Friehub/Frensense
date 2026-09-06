// [frensense]
// observation: `useImperativeHandle` exposes a setter for `innerHTML`, allowing a parent component to inject arbitrary HTML into the child
// impact: cross-site scripting (XSS) — parent can set malicious HTML like `<img src=x onerror=alert(1)>`
// improvement: expose only safe setters like `textContent` or validate HTML before assignment

'use client'

import { forwardRef, useImperativeHandle, useRef } from 'react'
import type { Ref } from 'react'

export interface PreviewHandle {
  setText: (text: string) => void
}

function Preview(_props: Record<string, never>, ref: Ref<PreviewHandle>) {
  const divRef = useRef<HTMLDivElement>(null)

  useImperativeHandle(ref, () => ({
    // SAFE: textContent prevents HTML injection
    setText(text: string) {
      if (divRef.current) {
        divRef.current.textContent = text
      }
    },
  }))

  return <div ref={divRef} />
}

export default function Parent() {
  const previewRef = useRef<PreviewHandle>(null)

  function handleUserContent(content: string) {
    previewRef.current?.setText(content)
  }

  return (
    <div>
      <textarea onChange={(e) => handleUserContent(e.target.value)} />
      <Preview ref={previewRef} />
    </div>
  )
}
