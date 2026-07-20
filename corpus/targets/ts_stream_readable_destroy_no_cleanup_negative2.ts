// SAFE alternative: Use fs.createReadStream which handles cleanup internally
import { createReadStream } from 'node:fs';
import { Readable } from 'node:stream';

function openFile(path: string): Readable {
  return createReadStream(path);
}
