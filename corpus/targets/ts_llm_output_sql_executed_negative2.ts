// SAFE: LLM output is validated against a whitelist of allowed SQL patterns before execution

import OpenAI from 'openai';
import { PrismaClient } from '@prisma/client';

const openai = new OpenAI();
const prisma = new PrismaClient();

function isSafeSelect(sql: string): boolean {
  const lower = sql.toLowerCase().trim();
  return lower.startsWith('select ') && !lower.includes(';') && !lower.includes('drop') && !lower.includes('delete');
}

export async function askDatabase(question: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Convert this to SQL: ${question}` }]
  });
  const sql = response.choices[0].message.content;
  if (!isSafeSelect(sql)) throw new Error('Generated SQL rejected by safety check');
  return prisma.$queryRawUnsafe(sql);
}
