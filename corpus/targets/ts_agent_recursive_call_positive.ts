// [frensense]
// observation: An agent tool's output triggers another agent call without any recursion limit, potentially creating an infinite loop.
// impact: A single user request can trigger unbounded nested agent calls, exhausting API credits and causing a denial of service.
// improvement: Add a recursion depth counter or maximum iteration limit for agent-to-agent calls.
// cwe: CWE-754
// cvss: 6.5
// owasp: 
// severity: Medium

import OpenAI from 'openai';

const openai = new OpenAI();

export async function researchAgent(topic: string): Promise<string> {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Research ${topic}. If you need more info, reply with "DEEPER: <subtopic>"` }]
  });
  const content = response.choices[0].message.content || '';
  if (content.startsWith('DEEPER:')) {
    const subtopic = content.replace('DEEPER:', '').trim();
    return researchAgent(subtopic);
  }
  return content;
}
