// [frensense]
// observation: `forwardRef` is used without setting `displayName` on the returned component
// impact: React DevTools shows `<ForwardRef>` or an anonymous component name instead of the meaningful component name, making debugging difficult in large component trees
// improvement: set `displayName` on the component returned by forwardRef

'use client'

import { forwardRef } from 'react'

const FancyInput = forwardRef<HTMLInputElement, { label: string }>(
  ({ label }, ref) => (
    <div>
      <label>{label}</label>
      <input ref={ref} />
    </div>
  ),
)

export default function Form() {
  return (
    <form>
      <FancyInput label="Name" />
      <FancyInput label="Email" />
    </form>
  )
}
