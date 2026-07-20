// SAFE: Model name is enforced server-side from an allowlist; user-supplied model is ignored

const ALLOWED_MODELS = ['gpt-4', 'gpt-4-turbo', 'gpt-3.5-turbo'];

import OpenAI from 'openai';

const openai = new OpenAI();

export async function chat(body: { model: string, message: string }) {
  const model = ALLOWED_MODELS.includes(body.model) ? body.model : ALLOWED_MODELS[0];
  const response = await openai.chat.completions.create({
    model,
    messages: [{ role: 'user', content: body.message }]
  });
  return response.choices[0].message.content;
}
