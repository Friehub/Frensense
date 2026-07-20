// [frensense]
// observation: A Server Action mutates data without checking the Origin or Referer header, making it vulnerable to cross-site request forgery via external form submissions.
// impact: An attacker on an external site can submit a form that triggers this server action, performing state-changing operations as the victim user.
// improvement: Validate the Origin or Referer header against the application's origin at the top of every server action.

'use server'

import { sql } from '@vercel/postgres'

export async function updateProfile(formData: FormData) {
  const userId = formData.get('userId') as string
  const email = formData.get('email') as string

  await sql`
    UPDATE users SET email = ${email} WHERE id = ${userId}
  `
}
