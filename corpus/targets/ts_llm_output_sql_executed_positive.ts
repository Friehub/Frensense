// [frensense]
// observation: LLM-generated SQL is executed directly against the database without validation or parameterization.
// impact: An attacker who influences the LLM output can perform SQL injection, extracting or modifying database contents.
// improvement: Validate LLM output against an expected schema before executing. Use parameterized queries, never raw model-generated SQL.
// cwe: CWE-20
// cvss: 7.5
// owasp: 
// severity: High

import OpenAI from 'openai';
import { PrismaClient } from '@prisma/client';

const openai = new OpenAI();
const prisma = new PrismaClient();

export async function askDatabase(question: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Convert this to SQL: ${question}` }]
  });
  const sql = response.choices[0].message.content;
  return prisma.$queryRawUnsafe(sql);
}
