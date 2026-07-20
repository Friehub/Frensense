// SAFE: FormData is validated using zod, and sensitive fields are explicitly checked for type safety

'use server'

import { z } from 'zod'
import prisma from '@/lib/prisma'
import { redirect } from 'next/navigation'
import { revalidatePath } from 'next/cache'

const updateSchema = z.object({
  userId: z.string().uuid(),
  email: z.string().email(),
  avatar: z.instanceof(File).optional(),
})

export async function updateProfile(formData: FormData) {
  const parsed = updateSchema.parse({
    userId: formData.get('userId'),
    email: formData.get('email'),
    avatar: formData.get('avatar') ?? undefined,
  })

  const data: Record<string, unknown> = { email: parsed.email }
  if (parsed.avatar && parsed.avatar.size > 0) {
    data.avatarUrl = await uploadAvatar(parsed.avatar)
  }

  await prisma.user.update({
    where: { id: parsed.userId },
    data,
  })

  revalidatePath('/profile')
  redirect('/profile')
}

async function uploadAvatar(file: File): Promise<string> {
  const buffer = Buffer.from(await file.arrayBuffer())
  return `/avatars/${crypto.randomUUID()}.${file.name.split('.').pop()}`
}
