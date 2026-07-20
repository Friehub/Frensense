// SAFE: LLM output is never executed — the model only returns text for display purposes

import OpenAI from 'openai';

const openai = new OpenAI();

export async function generateScriptSuggestion(prompt: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Write a bash script to ${prompt}` }]
  });
  return { suggestion: response.choices[0].message.content };
}
