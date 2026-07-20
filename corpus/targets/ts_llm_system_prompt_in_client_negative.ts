// SAFE: System prompts are stored only on the server and never exposed to the client

import OpenAI from 'openai';

const openai = new OpenAI();

const SYSTEM_PROMPT = 'You are a financial advisor. Never recommend stocks under $5. Always disclose risks.';

export async function chat(message: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: SYSTEM_PROMPT },
      { role: 'user', content: message }
    ]
  });
  return { reply: response.choices[0].message.content };
}
