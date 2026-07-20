// SAFE: content sanitized with DOMPurify before being set as innerHTML

import { Component, createSignal } from 'solid-js'
import DOMPurify from 'dompurify'

interface PreviewProps {
  content: string
}

const RichPreview: Component<PreviewProps> = (props) => {
  const [html, setHtml] = createSignal('')

  function loadPreview(id: string) {
    fetch(`/api/previews/${id}`)
      .then(r => r.json())
      .then(data => {
        const clean = DOMPurify.sanitize(data.html)
        setHtml(clean)
      })
  }

  return <div innerHTML={html()} />
}

export default RichPreview
