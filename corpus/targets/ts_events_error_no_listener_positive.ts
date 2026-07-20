// [frensense]
// observation: 'error' event emitted on an EventEmitter without any 'error' listener registered.
// impact: Node.js throws the error as an unhandled exception, crashing the process and terminating all connections.
// improvement: Always register an 'error' listener on EventEmitters, or use a domain/catch-all handler.

import { EventEmitter } from 'node:events';
import { createServer } from 'node:http';

class ConnectionPool extends EventEmitter {
  acquire(): void {
    if (this.listenerCount('error') === 0) {
      this.emit('error', new Error('Pool exhausted'));
    }
  }
}

function startServer(): void {
  const pool = new ConnectionPool();
  pool.acquire();
}
