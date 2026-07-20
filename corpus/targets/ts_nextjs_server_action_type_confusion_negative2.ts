// SAFE: The update uses a validated zod schema that strips unknown fields before the database write

'use server'

import { z } from 'zod'
import prisma from '@/lib/prisma'

const updateSchema = z.object({
  name: z.string().min(1).max(100).optional(),
  email: z.string().email().optional(),
  bio: z.string().max(500).optional(),
})

export async function updateProfile(formData: FormData) {
  const userId = formData.get('userId') as string
  const raw: Record<string, unknown> = {}

  for (const [key, value] of formData.entries()) {
    if (key !== 'userId') {
      raw[key] = value
    }
  }

  const parsed = updateSchema.parse(raw)

  await prisma.user.update({
    where: { id: userId },
    data: parsed,
  })
}
