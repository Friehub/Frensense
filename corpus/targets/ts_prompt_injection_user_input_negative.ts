// SAFE: User input is placed in a separate user-message role and never interpolated into the system prompt

import OpenAI from 'openai';

const openai = new OpenAI();

export async function chatHandler(userMessage: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: 'You are a helpful assistant.' },
      { role: 'user', content: userMessage }
    ]
  });
  return response.choices[0].message.content;
}
