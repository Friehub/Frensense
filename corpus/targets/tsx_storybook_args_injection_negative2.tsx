// [frensense]
// observation: Storybook args from URL query parameters are passed directly to a React component without sanitization, enabling XSS via the story preview
// impact: cross-site scripting (XSS) — attacker crafts a storybook URL with malicious args that execute arbitrary JavaScript in the preview iframe
// improvement: sanitize or validate component args before rendering, or use a safe wrapper that escapes HTML

'use client'

import { useEffect, useState } from 'react'
import DOMPurify from 'dompurify'

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
      // SAFE: user content is sanitized before being set as HTML
      content: DOMPurify.sanitize(params.get('content') ?? ''),
    })
  }, [])

  return (
    <div>
      <h1>{args.title}</h1>
      <div dangerouslySetInnerHTML={{ __html: args.content }} />
    </div>
  )
}
