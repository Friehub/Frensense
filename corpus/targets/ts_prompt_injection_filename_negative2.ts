// SAFE: Filenames are replaced with an auto-generated hash identifier, preventing injection via metadata

import crypto from 'crypto';

export async function processUpload(filename: string, content: string) {
  const fileId = crypto.createHash('md5').update(filename + Date.now()).digest('hex').slice(0, 8);
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: 'You are a document summarizer.' },
      { role: 'user', content: `File "${fileId}" contains:\n${content}\n\nSummarize this file.` }
    ]
  });
  return response.choices[0].message.content;
}
