// SAFE: sanitizes user content before setting innerHTML

import { forwardRef } from 'react'
import DOMPurify from 'dompurify'

interface Props {
  content: string
}

const DisplayBox = forwardRef<HTMLDivElement, Props>(({ content }, ref) => {
  const sanitized = DOMPurify.sanitize(content)
  return <div ref={ref} dangerouslySetInnerHTML={{ __html: sanitized }} />
})

DisplayBox.displayName = 'DisplayBox'

export default function Page() {
  return <DisplayBox content='<img src=x onerror="alert(1)">' />
}
