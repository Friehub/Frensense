// SAFE: The redirect target is validated against an allowlist of known paths before the redirect, and `redirect` is used instead of `permanentRedirect`

import { redirect } from 'next/navigation'

const ALLOWED_TARGETS = new Set(['/', '/dashboard', '/settings', '/profile'])

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const target = searchParams.get('dest')

  if (!target || !ALLOWED_TARGETS.has(target)) {
    redirect('/')
    return
  }
  redirect(target)
}
