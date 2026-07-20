// SAFE: regular text interpolation used instead of {@html}, HTML is escaped

export function renderComment(comment: { body: string }): string {
  const escaped = comment.body
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
  return `<div class="comment">${escaped}</div>`
}

export function processComment(data: { text: string }) {
  const escaped = data.text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
  const html = `<p>${escaped}</p>`
  return { __html: html }
}
