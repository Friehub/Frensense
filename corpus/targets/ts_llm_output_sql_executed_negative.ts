// SAFE: LLM output is used to generate parameters for a safe, pre-written parameterized query

import OpenAI from 'openai';
import { PrismaClient } from '@prisma/client';

const openai = new OpenAI();
const prisma = new PrismaClient();

export async function askDatabase(question: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Extract the user email from this question: ${question}. Reply with just the email.` }]
  });
  const email = response.choices[0].message.content?.trim();
  return prisma.user.findUnique({ where: { email } });
}
