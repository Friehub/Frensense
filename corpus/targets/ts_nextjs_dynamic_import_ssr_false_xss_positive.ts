// [frensense]
// observation: dynamic import with `ssr: false` renders a component that contains user-controlled HTML content
// impact: SSR is disabled so the dangerous HTML is rendered only on the client — but XSS still executes because no sanitization occurs before `dangerouslySetInnerHTML`
// improvement: sanitize user content before rendering, or use SSR with proper escaping

import dynamic from 'next/dynamic'
import { useState } from 'react'

const UserContent = dynamic(() => import('./user-content'), { ssr: false })

export default function Page() {
  const [html] = useState('<script>alert("xss")</script><p>hello</p>')
  return <UserContent html={html} />
}
