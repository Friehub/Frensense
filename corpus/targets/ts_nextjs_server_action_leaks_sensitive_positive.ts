// [frensense]
// observation: A server action returns the full database result object directly, potentially including sensitive fields like passwordHash or internal IDs.
// impact: Sensitive data is returned to the client and can be inspected in network responses or React dev tools.
// improvement: Select only the fields needed for the client response and never return raw database objects.

'use server';

import prisma from '@/lib/prisma';

export async function getUserProfile(userId: string) {
  const user = await prisma.user.findUnique({ where: { id: userId } });
  return user;
}
