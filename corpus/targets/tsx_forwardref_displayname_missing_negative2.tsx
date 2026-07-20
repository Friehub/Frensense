// SAFE: uses a named function expression for forwardRef, which automatically gets a displayName

'use client'

import { forwardRef } from 'react'

const FancyInput = forwardRef<HTMLInputElement, { label: string }>(
  function FancyInput({ label }, ref) {
    return (
      <div>
        <label>{label}</label>
        <input ref={ref} />
      </div>
    )
  },
)

export default function Form() {
  return (
    <form>
      <FancyInput label="Name" />
      <FancyInput label="Email" />
    </form>
  )
}
