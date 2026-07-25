// [frensense]
// observation: Writable stream write() return value (false = backpressure) is ignored; the caller continues writing regardless.
// impact: Internal buffer grows unbounded, causing memory exhaustion and eventual process termination.
// improvement: Check the write() return value and wait for the 'drain' event before continuing.
// cwe: CWE-284
// cvss: 8.8
// owasp: A01:2021
// severity: High

import { Writable } from 'node:stream';

function writeFast(dest: Writable, data: Buffer): void {
  dest.write(data);
}

function floodStream(dest: Writable, chunks: Buffer[]): void {
  for (const chunk of chunks) {
    dest.write(chunk);
  }
}
