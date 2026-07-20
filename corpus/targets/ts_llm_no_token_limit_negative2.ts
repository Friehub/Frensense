// SAFE: max_tokens is set via a config constant and enforced across all LLM calls

const LLM_CONFIG = { maxTokens: 500, temperature: 0.7 };

import OpenAI from 'openai';

const openai = new OpenAI();

export async function summarize(text: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Summarize: ${text}` }],
    max_tokens: LLM_CONFIG.maxTokens
  });
  return response.choices[0].message.content;
}
