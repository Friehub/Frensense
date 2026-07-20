// SAFE: Wrap listener in try/catch and emit 'error' event
import { EventEmitter } from 'node:events';

class Database extends EventEmitter {
  connect(): void {
    try {
      this.emit('connecting');
    } catch (err) {
      this.emit('error', err);
    }
  }
}

function safeHandler(db: Database): void {
  db.on('connecting', () => {
    try {
      if (!db.listenerCount('error')) {
        throw new Error('No error handler attached');
      }
    } catch (err) {
      db.emit('error', err);
    }
  });
}
