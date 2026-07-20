// [frensense]
// observation: A user-uploaded filename is injected into the LLM prompt, allowing metadata-based injection.
// impact: An attacker can embed instructions in a filename that override the LLM's intended behavior when processing uploaded files.
// improvement: Sanitize or strip filenames before including them in prompts, or use a hash-based identifier.

import OpenAI from 'openai';

const openai = new OpenAI();

export async function processUpload(filename: string, content: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'user', content: `File "${filename}" contains:\n${content}\n\nSummarize this file.` }
    ]
  });
  return response.choices[0].message.content;
}
