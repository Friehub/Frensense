// [frensense]
// observation: User-controlled input is directly interpolated into an LLM system or user prompt without sanitization.
// impact: Attackers can inject system-level instructions via user input, overriding the intended prompt behavior (prompt injection).
// improvement: Isolate user input from system instructions using delimiter boundaries, input validation, or dedicated user message roles.

import OpenAI from 'openai';

const openai = new OpenAI();

export async function chatHandler(userMessage: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: `You are a helpful assistant. User says: ${userMessage}` },
      { role: 'user', content: userMessage }
    ]
  });
  return response.choices[0].message.content;
}
