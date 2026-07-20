// SAFE: input validated with zod schema before database write

import { fail } from '@sveltejs/kit'
import type { Actions } from './$types'
import { db } from '$lib/database'
import { z } from 'zod'

const profileSchema = z.object({
  email: z.string().email(),
  bio: z.string().max(500)
})

export const actions: Actions = {
  default: async ({ request }) => {
    const data = await request.formData()
    const raw = {
      email: data.get('email') as string,
      bio: data.get('bio') as string
    }

    const parsed = profileSchema.safeParse(raw)
    if (!parsed.success) {
      return fail(400, { errors: parsed.error.flatten().fieldErrors })
    }

    await db.user.create({
      data: parsed.data
    })

    return { success: true }
  }
}
