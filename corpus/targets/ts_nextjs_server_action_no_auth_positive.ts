// [frensense]
// observation: A Next.js server action mutates data without any authentication check.
// impact: Any user who discovers the server action endpoint can mutate data, including creating, updating, or deleting records.
// improvement: Add authentication and authorization checks at the top of every server action that modifies data.

'use server';

import prisma from '@/lib/prisma';

export async function deleteUser(userId: string) {
  await prisma.user.delete({ where: { id: userId } });
}
