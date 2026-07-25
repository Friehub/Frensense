// [frensense]
// observation: Portal content (modal/tooltip) is rendered with a fixed z-index value that conflicts with other portal layers, causing stacking order issues
// impact: clickjacking — lower-z-index portal content can be transparently overlaid by higher-z-index content, tricking users into clicking unintended elements
// improvement: use a z-index management system (e.g., stack context counter) to ensure proper stacking order
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium

'use client'

import { createPortal, useState } from 'react'

function Modal({ onClose, children }: { onClose: () => void; children: React.ReactNode }) {
  return createPortal(
    <div style={{ position: 'fixed', inset: 0, zIndex: 100, background: 'rgba(0,0,0,0.5)' }}>
      <div style={{ background: 'white', padding: 20 }}>
        {children}
        <button onClick={onClose}>Close</button>
      </div>
    </div>,
    document.body,
  )
}

function Tooltip({ text }: { text: string }) {
  return createPortal(
    <div style={{ position: 'fixed', zIndex: 200, background: 'yellow' }}>
      {text}
    </div>,
    document.body,
  )
}

export default function App() {
  const [showModal, setShowModal] = useState(false)

  return (
    <div>
      <button onClick={() => setShowModal(true)}>Open Modal</button>
      {showModal && (
        <Modal onClose={() => setShowModal(false)}>
          <Tooltip text="This tooltip is above the modal overlay due to higher z-index" />
          <p>Modal content</p>
        </Modal>
      )}
    </div>
  )
}
