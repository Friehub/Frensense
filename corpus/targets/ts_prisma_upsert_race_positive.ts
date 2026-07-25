// [frensense]
// observation: Prisma upsert() is used on a non-unique field, allowing duplicate record creation in concurrent requests.
// impact: Race conditions can create duplicate records despite the upsert, violating uniqueness assumptions.
// improvement: Ensure upsert uses a unique field constraint, or use findFirst + transaction + create with unique constraint.
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function upsertUser(email: string, name: string) {
  return prisma.user.upsert({
    where: { email },
    update: { name },
    create: { email, name }
  });
}
