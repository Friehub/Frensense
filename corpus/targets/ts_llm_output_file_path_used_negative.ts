// SAFE: File operations are restricted to a sandbox directory with path traversal prevented

import OpenAI from 'openai';
import fs from 'fs/promises';
import path from 'path';

const ALLOWED_DIR = '/data/sandbox';

function isPathSafe(requestedPath: string): boolean {
  const resolved = path.resolve(ALLOWED_DIR, requestedPath);
  return resolved.startsWith(ALLOWED_DIR);
}

const openai = new OpenAI();

export async function readFileSuggestion(prompt: string) {
  const response = await openai.chat.completions.create({
    model: 'gpt-4',
    messages: [{ role: 'user', content: `What file should I read to understand ${prompt}?` }]
  });
  const filePath = response.choices[0].message.content?.trim();
  if (!filePath || !isPathSafe(filePath)) throw new Error('Invalid file path');
  const content = await fs.readFile(path.join(ALLOWED_DIR, filePath), 'utf-8');
  return content;
}
