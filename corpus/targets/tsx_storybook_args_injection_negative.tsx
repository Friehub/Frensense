// [frensense]
// observation: Storybook args from URL query parameters are passed directly to a React component without sanitization, enabling XSS via the story preview
// impact: cross-site scripting (XSS) — attacker crafts a storybook URL with malicious args that execute arbitrary JavaScript in the preview iframe
// improvement: sanitize or validate component args before rendering, or use a safe wrapper that escapes HTML

'use client'

import { useEffect, useState } from 'react'

interface StoryArgs {
  title: string
  content: string
}

export default function StorybookPreview() {
  const [args, setArgs] = useState<StoryArgs>({ title: '', content: '' })

  useEffect(() => {
    const params = new URLSearchParams(window.location.search)
    setArgs({
      title: params.get('title') ?? '',
      content: params.get('content') ?? '',
    })
  }, [])

  return (
    <div>
      {/* SAFE: React escapes text content automatically */}
      <h1>{args.title}</h1>
      {/* SAFE: dangerouslySetInnerHTML is not used; content is rendered as text */}
      <div>{args.content}</div>
    </div>
  )
}
