// [frensense]
// observation: Retrieved RAG document content is injected directly into the LLM prompt without separating it from system instructions.
// impact: An attacker who controls the content of a retrieved document can inject instructions into the LLM, overriding the intended behavior.
// improvement: Wrap retrieved document content in delimiters or a dedicated content role, and validate or strip instruction-like sequences.

import OpenAI from 'openai';

const openai = new OpenAI();

export async function ragQuery(userQuery: string, documents: string[]) {
  const context = documents.join('\n\n');
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: `Answer based on context:\n${context}\n\nQuery: ${userQuery}` }
    ]
  });
  return response.choices[0].message.content;
}
