// SAFE alternative: Use for-await-of to consume readable
import { createReadStream } from 'node:fs';
import { Readable } from 'node:stream';

async function collectContent(path: string): Promise<string> {
  const src = createReadStream(path, { encoding: 'utf8' });
  let content = '';
  for await (const chunk of src) {
    content += chunk;
  }
  return content;
}
