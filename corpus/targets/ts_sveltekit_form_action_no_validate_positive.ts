// [frensense]
// observation: SvelteKit form action stores user input in database without validation
// impact: malformed or malicious data can be persisted, leading to stored XSS, SQL injection, or data corruption
// improvement: validate and sanitize all input fields before database operations

import { fail } from '@sveltejs/kit'
import type { Actions } from './$types'
import { db } from '$lib/database'

export const actions: Actions = {
  default: async ({ request }) => {
    const data = await request.formData()
    const email = data.get('email') as string
    const bio = data.get('bio') as string

    await db.user.create({
      data: { email, bio }
    })

    return { success: true }
  }
}
