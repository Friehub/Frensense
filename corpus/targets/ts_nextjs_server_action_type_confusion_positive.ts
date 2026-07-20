// [frensense]
// observation: A Server Action accepts a FormData entry called `metadata` and spreads it directly into a database record without checking whether the keys collide with protected fields like `role`, `isAdmin`, or `balance`.
// impact: An attacker can add extra fields to the FormData payload that override protected columns in the database (e.g., `role: "admin"`), achieving privilege escalation via type confusion.
// improvement: Never spread raw FormData into database updates. Use a strict allowlist of updatable fields.

'use server'

import prisma from '@/lib/prisma'

export async function updateProfile(formData: FormData) {
  const userId = formData.get('userId') as string
  const data: Record<string, unknown> = {}

  for (const [key, value] of formData.entries()) {
    if (key !== 'userId') {
      data[key] = value
    }
  }

  await prisma.user.update({
    where: { id: userId },
    data,
  })
}
