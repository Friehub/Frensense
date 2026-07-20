// SAFE: use proper autocomplete values for password fields
export function renderLoginForm(): string {
  return `
    <form method="POST" action="/login">
      <label>Username</label>
      <input type="text" name="username" autocomplete="username" />
      <label>Password</label>
      <input type="password" name="password" autocomplete="current-password" />
      <button type="submit">Log in</button>
    </form>
  `;
}

export function renderSignupForm(): string {
  return `
    <form method="POST" action="/signup">
      <input type="password" name="password" autocomplete="new-password" />
      <input type="password" name="confirm" autocomplete="new-password" />
    </form>
  `;
}
