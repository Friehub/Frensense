// SAFE: Only fields in the explicit allowlist are extracted from FormData; all other fields are ignored

'use server'

import prisma from '@/lib/prisma'

const UPDATABLE_FIELDS = new Set(['name', 'email', 'bio', 'avatarUrl'])

export async function updateProfile(formData: FormData) {
  const userId = formData.get('userId') as string
  const data: Record<string, unknown> = {}

  for (const [key, value] of formData.entries()) {
    if (UPDATABLE_FIELDS.has(key)) {
      data[key] = value
    }
  }

  if (Object.keys(data).length === 0) {
    throw new Error('no updatable fields provided')
  }

  await prisma.user.update({
    where: { id: userId },
    data,
  })
}
