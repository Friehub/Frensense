// SAFE: displayName is set on the forwarded component for DevTools identification

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

FancyInput.displayName = 'FancyInput'

export default function Form() {
  return (
    <form>
      <FancyInput label="Name" />
      <FancyInput label="Email" />
    </form>
  )
}
