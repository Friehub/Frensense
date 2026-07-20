// SAFE: portal renders into a container that is a child of the app root

'use client'

import { createPortal } from 'react-dom'

export default function Modal() {
  const target = document.getElementById('modal-root')
  if (!target) return null
  return createPortal(
    <div className="modal">
      <p>Sensitive user data</p>
    </div>,
    target
  )
}
