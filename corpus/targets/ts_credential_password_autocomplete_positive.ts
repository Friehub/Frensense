// [frensense]
// observation: A password field uses autocomplete="off" to prevent the browser from storing the password. However, modern browsers increasingly ignore autocomplete="off" on password fields for security reasons, or the attribute is only applied to the form but not the individual input.
// impact: The autocomplete="off" attribute does not reliably prevent password storage in modern browsers. Users may still be prompted to save passwords, and the false sense of security leads developers to believe passwords are protected when they are not.
// improvement: Use autocomplete="new-password" for password creation fields and autocomplete="current-password" for login fields. Never use autocomplete="off" for password fields.
// cwe: CWE-521
// cvss: 7.5
// owasp: A07:2021
// severity: High

export function renderLoginForm(): string {
  return `
    <form method="POST" action="/login" autocomplete="off">
      <label>Username</label>
      <input type="text" name="username" />
      <label>Password</label>
      <input type="password" name="password" />
      <button type="submit">Log in</button>
    </form>
  `;
}

export function renderSignupForm(): string {
  return `
    <form method="POST" action="/signup">
      <input type="password" name="password" autocomplete="off" />
      <input type="password" name="confirm" autocomplete="off" />
    </form>
  `;
}
