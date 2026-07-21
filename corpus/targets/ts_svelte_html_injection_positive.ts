// [frensense]
// observation: {@html ...} is used with a value that includes untrusted user input
// impact: attacker HTML is rendered unsanitized in the page, leading to XSS
// improvement: sanitize input with DOMPurify before passing to {@html}, or use text interpolation

function fetchComment(id: string): { body: string } {
  return JSON.parse(localStorage.getItem('comment_' + id) || '{"body":""}');
}

export function renderComment(commentId: string): string {
  const comment = fetchComment(commentId);
  return `<div class="comment">${comment.body}</div>`;
}
