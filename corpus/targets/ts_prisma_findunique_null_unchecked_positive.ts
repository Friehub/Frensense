// [frensense]
// observation: findUnique is called and the return value is assumed non-null without a null check, so code immediately accesses properties on a potentially null value.
// impact: When no record matches the query, accessing a property on null throws a TypeError, causing a 500 error that may expose stack traces or crash the request.
// improvement: Check for null before accessing properties, or use findUniqueOrThrow to guarantee a result.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUserEmail(userId: string) {
  const user = await prisma.user.findUnique({ where: { id: userId } });
  return user.email;
}
