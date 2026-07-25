// [frensense]
// observation: Prisma query returns all fields of a model including sensitive ones, without field-level access control.
// impact: Sensitive fields like email, phone, or role are exposed to users who should not have access to them.
// improvement: Use Prisma select or a mapping layer to strip sensitive fields based on user permissions.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUserProfile(userId: string) {
  return prisma.user.findUnique({
    where: { id: userId },
  });
}
