// SAFE alternative: Check listenerCount before emitting 'error'
import { EventEmitter } from 'node:events';

class ConnectionPool extends EventEmitter {
  acquire(): void {
    const err = new Error('Pool exhausted');
    if (this.listenerCount('error') > 0) {
      this.emit('error', err);
    } else {
      console.error('Fatal:', err.message);
    }
  }
}

function startServer(): void {
  const pool = new ConnectionPool();
  pool.on('error', (err) => {
    console.error('Connection pool error:', err.message);
  });
  pool.acquire();
}
