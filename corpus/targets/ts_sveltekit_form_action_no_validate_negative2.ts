// SAFE: manual field-level validation before database write

import { fail } from '@sveltejs/kit'
import type { Actions } from './$types'
import { db } from '$lib/database'

function isValidEmail(email: string): boolean {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
}

export const actions: Actions = {
  default: async ({ request }) => {
    const data = await request.formData()
    const email = data.get('email') as string
    const bio = data.get('bio') as string

    const errors: Record<string, string> = {}

    if (!isValidEmail(email)) {
      errors.email = 'Invalid email format'
    }

    if (typeof bio !== 'string' || bio.length > 500) {
      errors.bio = 'Bio must be at most 500 characters'
    }

    if (Object.keys(errors).length > 0) {
      return fail(400, { errors })
    }

    await db.user.create({
      data: { email: email.trim().toLowerCase(), bio }
    })

    return { success: true }
  }
}
