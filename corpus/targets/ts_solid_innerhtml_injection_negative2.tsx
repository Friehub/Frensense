// SAFE: textContent used instead of innerHTML; HTML is escaped

import { Component, createSignal } from 'solid-js'

interface PreviewProps {
  content: string
}

function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;')
}

const RichPreview: Component<PreviewProps> = (props) => {
  const [text, setText] = createSignal('')

  function loadPreview(id: string) {
    fetch(`/api/previews/${id}`)
      .then(r => r.json())
      .then(data => {
        setText(escapeHtml(data.html))
      })
  }

  return <div textContent={text()} />
}

export default RichPreview
