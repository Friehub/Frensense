// SAFE: The redirect target is verified to be a relative URL (starts with '/'), preventing open redirect to external domains

import { redirect } from 'next/navigation'

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const target = searchParams.get('dest')

  if (!target || !target.startsWith('/')) {
    redirect('/')
    return
  }
  redirect(target)
}
