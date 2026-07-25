// [frensense]
// observation: React 19 form action does not include or validate a CSRF token — the action is a plain server action callable cross-origin
// impact: cross-site request forgery — an external site can submit the form on behalf of the authenticated user
// improvement: use Next.js server actions with CSRF protection or include and verify a double-submit cookie pattern
// cwe: CWE-352
// cvss: 8.8
// owasp: A01:2021
// severity: High

'use client'

export default function TransferForm() {
  return (
    <form action="/api/transfer" method="POST">
      <input name="amount" type="number" />
      <input name="toAccount" type="text" />
      <button type="submit">Transfer</button>
    </form>
  )
}
