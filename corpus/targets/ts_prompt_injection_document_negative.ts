// SAFE: Retrieved documents are wrapped in XML-style delimiters to separate them from system instructions

import OpenAI from 'openai';

const openai = new OpenAI();

export async function ragQuery(userQuery: string, documents: string[]) {
  const context = documents.map((doc, i) => `<document index="${i}">\n${doc}\n</document>`).join('\n\n');
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: 'You are a RAG assistant. Answer based only on the documents below.' },
      { role: 'user', content: `${context}\n\nQuery: ${userQuery}` }
    ]
  });
  return response.choices[0].message.content;
}
