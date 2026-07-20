// SAFE: Recursive agent calls are limited to a maximum depth of 3

import OpenAI from 'openai';

const openai = new OpenAI();

export async function researchAgent(topic: string, depth = 0): Promise<string> {
  if (depth >= 3) throw new Error('Max research depth reached');
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Research ${topic}. If you need more info, reply with "DEEPER: <subtopic>"` }]
  });
  const content = response.choices[0].message.content || '';
  if (content.startsWith('DEEPER:')) {
    const subtopic = content.replace('DEEPER:', '').trim();
    return researchAgent(subtopic, depth + 1);
  }
  return content;
}
