// SAFE: max_tokens is explicitly set to limit response length and prevent runaway costs

import OpenAI from 'openai';

const openai = new OpenAI();

export async function summarize(text: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Summarize: ${text}` }],
    max_tokens: 200
  });
  return response.choices[0].message.content;
}
