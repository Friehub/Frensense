// [frensense]
// observation: Solid.js innerHTML or setHTML prop is used with a value derived from user input
// impact: attacker-controlled HTML/scripts are rendered in the DOM, leading to XSS
// improvement: use textContent or sanitize with DOMPurify before setting innerHTML

import { Component, createSignal } from 'solid-js'

interface PreviewProps {
  content: string
}

const RichPreview: Component<PreviewProps> = (props) => {
  const [html, setHtml] = createSignal('')

  function loadPreview(id: string) {
    fetch(`/api/previews/${id}`)
      .then(r => r.json())
      .then(data => {
        setHtml(data.html)
      })
  }

  return <div innerHTML={html()} />
}

export default RichPreview
