// SAFE: Token is stored in an HttpOnly secure cookie, inaccessible to JavaScript
export async function handleLogin(form: HTMLFormElement): Promise<void> {
  const res = await fetch('/api/login', {
    method: 'POST',
    body: new FormData(form),
    credentials: 'include'
  });
  if (res.ok) window.location.href = '/dashboard';
}
