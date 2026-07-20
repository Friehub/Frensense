// [frensense]
// observation: A Next.js Server Action directly reads fields from FormData and passes them unsanitized into database operations and response redirects, trusting the client-provided content type and field structure.
// impact: An attacker can craft a multipart/form-data request with unexpected field types (e.g., array of file objects for a string field) causing type confusion, prototype pollution, or injection into server logic.
// improvement: Validate every FormData field type and value against a strict schema before processing.

'use server'

import prisma from '@/lib/prisma'
import { redirect } from 'next/navigation'
import { revalidatePath } from 'next/cache'

export async function updateProfile(formData: FormData) {
  const userId = formData.get('userId') as string
  const email = formData.get('email') as string
  const bio = formData.get('bio') as string

  await prisma.user.update({
    where: { id: userId },
    data: { email, bio },
  })

  revalidatePath('/profile')
  redirect('/profile')
}
