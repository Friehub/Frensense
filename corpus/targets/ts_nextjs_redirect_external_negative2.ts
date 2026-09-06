// SAFE: Only relative redirects are allowed; absolute URLs are rejected

import { redirect } from 'next/navigation';

export async function GET(request: Request) {
  const url = new URL(request.url);
  const dest = url.searchParams.get('redirect') || '/';
  if (dest.startsWith('http://') || dest.startsWith('https://')) {
    throw new Error('External redirects are not allowed');
  }
  redirect(dest);
}
