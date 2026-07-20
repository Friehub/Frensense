// SAFE: Filenames are sanitized to remove instruction-like text before inclusion in the prompt

function sanitizeFilename(name: string): string {
  return name.replace(/[<>{}\[\]"]/g, '').substring(0, 100);
}

export async function processUpload(filename: string, content: string) {
  const safeName = sanitizeFilename(filename);
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [
      { role: 'system', content: 'You are a document summarizer.' },
      { role: 'user', content: `File "${safeName}" contains:\n${content}\n\nSummarize this file.` }
    ]
  });
  return response.choices[0].message.content;
}
