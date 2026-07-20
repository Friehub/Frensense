// SAFE: LLM output is used as a search term, not a path — files are resolved from a pre-indexed catalog

const FILE_CATALOG: Record<string, string> = {
  'config': '/etc/app/config.json',
  'readme': '/etc/app/README.md',
  'logs': '/var/log/app/current.log',
};

export async function readFileSuggestion(prompt: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `Which of these files matches: config, readme, logs? Query: ${prompt}` }]
  });
  const key = response.choices[0].message.content?.trim().toLowerCase();
  const filePath = key ? FILE_CATALOG[key] : null;
  if (!filePath) throw new Error('Unknown file');
  const fs = await import('fs/promises');
  const content = await fs.readFile(filePath, 'utf-8');
  return content;
}
