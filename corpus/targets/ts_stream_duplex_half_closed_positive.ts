// [frensense]
// observation: Duplex stream half-closed (writable end ended) without handling the readable end finishing, causing a hang.
// impact: The readable side may never close, leaking resources and leaving consumers waiting indefinitely.
// improvement: Listen for 'end' on the readable side after calling end() on the writable side, or use pipeline.

import { Duplex } from 'node:stream';

function sendAndHang(duplex: Duplex, message: string): void {
  duplex.write(message);
  duplex.end();
}

function partialClose(stream: Duplex): void {
  stream.end('done');
}
