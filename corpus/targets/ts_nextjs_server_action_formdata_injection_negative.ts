// SAFE: FormData fields are validated with a schema before being used in any operation

'use server'

import { z } from 'zod'
import prisma from '@/lib/prisma'
import { redirect } from 'next/navigation'
import { revalidatePath } from 'next/cache'

const profileSchema = z.object({
  userId: z.string().uuid(),
  email: z.string().email(),
  bio: z.string().max(500),
})

export async function updateProfile(formData: FormData) {
  const raw = {
    userId: formData.get('userId'),
    email: formData.get('email'),
    bio: formData.get('bio'),
  }

  const parsed = profileSchema.parse(raw)

  await prisma.user.update({
    where: { id: parsed.userId },
    data: { email: parsed.email, bio: parsed.bio },
  })

  revalidatePath('/profile')
  redirect('/profile')
}
