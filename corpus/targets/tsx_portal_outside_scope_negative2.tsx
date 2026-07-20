// SAFE: renders inline without a portal, staying within the component tree

'use client'

export default function Modal() {
  return (
    <div className="modal-overlay">
      <div className="modal">
        <p>Sensitive user data</p>
      </div>
    </div>
  )
}
