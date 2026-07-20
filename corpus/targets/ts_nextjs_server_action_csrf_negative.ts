// SAFE: Server Action validates the Origin header against the application's origin before processing the mutation

'use server'

import { headers } from 'next/headers'
import { sql } from '@vercel/postgres'

const APP_ORIGIN = process.env.APP_ORIGIN ?? 'https://example.com'

function assertSameOrigin(requestOrigin: string | null): void {
  if (requestOrigin === null || requestOrigin !== APP_ORIGIN) {
    throw new Error('cross-origin request rejected')
  }
}

export async function updateProfile(formData: FormData) {
  const hdrs = await headers()
  assertSameOrigin(hdrs.get('origin'))

  const userId = formData.get('userId') as string
  const email = formData.get('email') as string

  await sql`
    UPDATE users SET email = ${email} WHERE id = ${userId}
  `
}
