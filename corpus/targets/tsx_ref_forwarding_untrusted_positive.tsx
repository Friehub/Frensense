// [frensense]
// observation: `forwardRef` passes a ref to an uncontrolled input that renders raw user content — the ref allows direct DOM manipulation to inject HTML
// impact: cross-site scripting — the ref callback can be used to set `innerHTML` on the element, bypassing React's escaping
// improvement: avoid forwarding refs to uncontrolled components that render user content; sanitize output instead

import { forwardRef } from 'react'

interface Props {
  content: string
}

const DisplayBox = forwardRef<HTMLDivElement, Props>(({ content }, ref) => {
  return <div ref={ref} dangerouslySetInnerHTML={{ __html: content }} />
})

DisplayBox.displayName = 'DisplayBox'

export default function Page() {
  return <DisplayBox content='<img src=x onerror="alert(1)">' />
}
