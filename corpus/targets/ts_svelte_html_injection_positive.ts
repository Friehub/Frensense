// [frensense]
// observation: {@html ...} is used with a value that includes untrusted user input
// impact: attacker HTML is rendered unsanitized in the page, leading to XSS
// improvement: sanitize input with DOMPurify before passing to {@html}, or use text interpolation

export function renderComment(comment: { body: string }): string {
  return `<div class="comment">${comment.body}</div>`
}

export function processComment(data: { text: string }) {
  const html = `<p>${data.text}</p>`
  return { __html: html }
}
