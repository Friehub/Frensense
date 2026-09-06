// SAFE: Handle both ends of the duplex stream
import { Duplex } from 'node:stream';

function sendAndFinish(duplex: Duplex, message: string): Promise<void> {
  return new Promise((resolve) => {
    duplex.on('end', resolve);
    duplex.write(message);
    duplex.end();
  });
}

function safePartialClose(stream: Duplex): Promise<void> {
  return new Promise((resolve) => {
    stream.on('finish', resolve);
    stream.end('done');
  });
}
