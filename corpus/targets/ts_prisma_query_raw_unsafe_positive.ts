// [frensense]
// observation: Prisma's $queryRawUnsafe is called with a SQL string that includes user-controlled input.
// impact: An attacker can perform SQL injection by providing input that alters the SQL query structure.
// improvement: Use $queryRaw with tagged template literals or $queryRawUnsafe with parameterized values via $queryRaw's template syntax.
// cwe: CWE-119
// cvss: 9.8
// owasp: 
// severity: Critical

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getUser(id: string) {
  const result = await prisma.$queryRawUnsafe(`SELECT * FROM users WHERE id = '${id}'`);
  return result;
}
