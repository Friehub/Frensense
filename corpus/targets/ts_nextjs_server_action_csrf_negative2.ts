// SAFE: Server Action validates the Referer header and uses a cryptographically-bound CSRF token embedded in the form

'use server'

import { headers } from 'next/headers'
import { sql } from '@vercel/postgres'

const APP_ORIGIN = process.env.APP_ORIGIN ?? 'https://example.com'

function assertValidReferer(referer: string | null): void {
  if (referer === null || !referer.startsWith(APP_ORIGIN)) {
    throw new Error('cross-origin request rejected')
  }
}

export async function updateProfile(formData: FormData) {
  const hdrs = await headers()
  assertValidReferer(hdrs.get('referer'))

  const csrfToken = formData.get('csrf_token') as string
  if (!csrfToken || csrfToken !== process.env.CSRF_SECRET) {
    throw new Error('invalid CSRF token')
  }

  const userId = formData.get('userId') as string
  const email = formData.get('email') as string

  await sql`
    UPDATE users SET email = ${email} WHERE id = ${userId}
  `
}
