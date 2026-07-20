// SAFE: user input sanitized with DOMPurify before being used in {@html}

import DOMPurify from 'dompurify'

export function renderComment(comment: { body: string }): string {
  const clean = DOMPurify.sanitize(comment.body)
  return `<div class="comment">${clean}</div>`
}

export function processComment(data: { text: string }) {
  const clean = DOMPurify.sanitize(data.text)
  const html = `<p>${clean}</p>`
  return { __html: html }
}
