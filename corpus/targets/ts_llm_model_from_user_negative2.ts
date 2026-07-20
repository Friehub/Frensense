// SAFE: Model selection is done entirely server-side; the client never specifies the model

import OpenAI from 'openai';

const openai = new OpenAI();

export async function chat(message: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4-turbo',
    messages: [{ role: 'user', content: message }]
  });
  return response.choices[0].message.content;
}
