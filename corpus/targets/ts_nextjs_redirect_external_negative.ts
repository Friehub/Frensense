// SAFE: Redirect is validated to be a relative path before being used

import { redirect } from 'next/navigation';

const ALLOWED_HOSTS = ['app.example.com'];

function isSafeRedirect(dest: string): boolean {
  try {
    const url = new URL(dest, 'http://localhost');
    if (url.pathname === dest) return true;
    return ALLOWED_HOSTS.includes(url.hostname);
  } catch {
    return false;
  }
}

export async function GET(request: Request) {
  const url = new URL(request.url);
  const dest = url.searchParams.get('redirect') || '/';
  if (!isSafeRedirect(dest)) throw new Error('Invalid redirect target');
  redirect(dest);
}
