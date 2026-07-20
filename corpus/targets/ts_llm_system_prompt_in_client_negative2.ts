// SAFE: The system prompt is loaded from an environment variable, never bundled in client code

import OpenAI from 'openai';

const openai = new OpenAI();
const SYSTEM_PROMPT = process.env.LLM_SYSTEM_PROMPT || 'You are a helpful assistant.';

export async function chat(message: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: SYSTEM_PROMPT },
      { role: 'user', content: message }
    ]
  });
  return { reply: response.choices[0].message.content, promptVersion: 'server-v2' };
}
