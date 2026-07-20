// SAFE: uses a global z-index counter to assign sequential z-index values, preventing stacking conflicts

'use client'

import { createPortal, useState } from 'react'

let globalZIndex = 1000

function nextZIndex() {
  return globalZIndex++
}

function Modal({ onClose, children }: { onClose: () => void; children: React.ReactNode }) {
  const [zIndex] = useState(() => nextZIndex())

  return createPortal(
    <div style={{ position: 'fixed', inset: 0, zIndex, background: 'rgba(0,0,0,0.5)' }}>
      <div style={{ background: 'white', padding: 20 }}>
        {children}
        <button onClick={onClose}>Close</button>
      </div>
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
          <p>Modal content</p>
        </Modal>
      )}
    </div>
  )
}
