// [frensense]
// observation: A synchronous error is thrown inside an event emitter listener without being wrapped in try/catch inside the emit.
// impact: The exception propagates up the call stack unpredictably, bypassing any surrounding error handling and often crashing the process.
// improvement: Wrap listener body in try/catch and emit an 'error' event instead of throwing.

import { EventEmitter } from 'node:events';

class Database extends EventEmitter {
  connect(): void {
    this.emit('connecting');
    throw new Error('Connection failed');
  }
}

function unsafeHandler(db: Database): void {
  db.on('connecting', () => {
    if (!db.listenerCount('error')) {
      throw new Error('No error handler attached');
    }
  });
}
