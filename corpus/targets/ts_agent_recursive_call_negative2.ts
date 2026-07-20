// SAFE: Agent calls use a loop with a hard iteration limit instead of unbounded recursion

import OpenAI from 'openai';

const openai = new OpenAI();

export async function researchAgent(initialTopic: string): Promise<string> {
  let topic = initialTopic;
  for (let i = 0; i < 5; i++) {
    const response = await openai.chat.completions.create({
      model: 'gpt-4',
      messages: [{ role: 'user', content: `Research ${topic}. If you need more info, reply with "DEEPER: <subtopic>"` }]
    });
    const content = response.choices[0].message.content || '';
    if (!content.startsWith('DEEPER:')) return content;
    topic = content.replace('DEEPER:', '').trim();
  }
  return 'Research incomplete — too many subtopics';
}
