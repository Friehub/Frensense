// [frensense]
// observation: An LLM API call is made without specifying the max_tokens parameter, allowing unbounded token usage.
// impact: An attacker can craft prompts that generate arbitrarily long responses, causing runaway API costs and potential denial of service.
// improvement: Always set a reasonable max_tokens limit on every LLM API call.

import OpenAI from 'openai';

const openai = new OpenAI();

export async function summarize(text: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Summarize: ${text}` }]
  });
  return response.choices[0].message.content;
}
