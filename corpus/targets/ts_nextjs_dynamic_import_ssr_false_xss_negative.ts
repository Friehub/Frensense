// SAFE: sanitizes user HTML with DOMPurify before rendering

import dynamic from 'next/dynamic'
import { useState } from 'react'
import DOMPurify from 'dompurify'

const UserContent = dynamic(() => import('./user-content'), { ssr: false })

export default function Page() {
  const [html] = useState('<script>alert("xss")</script><p>hello</p>')
  const sanitized = DOMPurify.sanitize(html)
  return <UserContent html={sanitized} />
}
