// SAFE alternative: use random autocomplete tokens to prevent password managers from matching the field
export function renderLoginForm(): string {
  const token = generateRandomToken();
  return `
    <form method="POST" action="/login">
      <label>Username</label>
      <input type="text" name="username" autocomplete="username" />
      <label>Password</label>
      <input type="password" name="password" autocomplete="off" />
      <input type="hidden" name="token" value="${token}" />
      <button type="submit">Log in</button>
    </form>
  `;
}

function generateRandomToken(): string {
  return Math.random().toString(36).substring(2, 15);
}

export function renderSignupForm(): string {
  return `
    <form method="POST" action="/signup">
      <input type="password" name="password" autocomplete="new-password" />
      <input type="password" name="confirm" autocomplete="new-password" />
    </form>
  `;
}
