// SAFE: renders content as text (React-escaped) instead of using innerHTML

import { forwardRef } from 'react'

interface Props {
  content: string
}

const DisplayBox = forwardRef<HTMLDivElement, Props>(({ content }, ref) => {
  return <div ref={ref}>{content}</div>
})

DisplayBox.displayName = 'DisplayBox'

export default function Page() {
  return <DisplayBox content='&lt;script&gt;alert(1)&lt;/script&gt;' />
}
