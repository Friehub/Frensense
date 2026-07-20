// [frensense]
// observation: The LLM model name is taken directly from the user's request, allowing selection of arbitrary models.
// impact: An attacker can select a less restricted or cheaper model, bypassing safety filters or incurring unexpected costs.
// improvement: Whitelist the allowed model names server-side and ignore the client's model selection.

import OpenAI from 'openai';

const openai = new OpenAI();

export async function chat(body: { model: string, message: string }) {
  const response = await openai.chat.completions.create({
    model: body.model,
    messages: [{ role: 'user', content: body.message }]
  });
  return response.choices[0].message.content;
}
