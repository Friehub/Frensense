// [frensense]
// observation: stream.pipeline called without a finished listener or error handler on the destination stream.
// impact: If the pipeline fails mid-stream, the error is unhandled, causing hanging resources and uncollected garbage.
// improvement: Use the callback argument of pipeline or listen for the 'close'/'error' events on the pipeline result.

import { createReadStream, createWriteStream } from 'node:fs';
import { pipeline } from 'node:stream';
import { createGunzip } from 'node:zlib';

function copyFile(src: string, dest: string): void {
  const read = createReadStream(src);
  const write = createWriteStream(dest);
  pipeline(read, write);
}

function ungzipAndSave(input: NodeJS.ReadableStream, dest: string): void {
  const write = createWriteStream(dest);
  const gunzip = createGunzip();
  pipeline(input, gunzip, write);
}
