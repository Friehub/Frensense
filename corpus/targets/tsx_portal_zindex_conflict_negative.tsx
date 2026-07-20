// SAFE: uses a shared z-index context to ensure proper stacking order across all portal instances

'use client'

import { createContext, createPortal, useContext, useState } from 'react'

const ZIndexContext = createContext(0)

function Modal({ onClose, children }: { onClose: () => void; children: React.ReactNode }) {
  const baseZ = useContext(ZIndexContext)
  return createPortal(
    <div style={{ position: 'fixed', inset: 0, zIndex: baseZ + 100, background: 'rgba(0,0,0,0.5)' }}>
      <div style={{ background: 'white', padding: 20 }}>
        <ZIndexContext.Provider value={baseZ + 10}>{children}</ZIndexContext.Provider>
        <button onClick={onClose}>Close</button>
      </div>
    </div>,
    document.body,
  )
}

function Tooltip({ text }: { text: string }) {
  const baseZ = useContext(ZIndexContext)
  return createPortal(
    <div style={{ position: 'fixed', zIndex: baseZ + 1, background: 'yellow' }}>
      {text}
    </div>,
    document.body,
  )
}

export default function App() {
  const [showModal, setShowModal] = useState(false)

  return (
    <ZIndexContext.Provider value={0}>
      <button onClick={() => setShowModal(true)}>Open Modal</button>
      {showModal && (
        <Modal onClose={() => setShowModal(false)}>
          <Tooltip text="Tooltip stays below modal overlay" />
          <p>Modal content</p>
        </Modal>
      )}
    </ZIndexContext.Provider>
  )
}
