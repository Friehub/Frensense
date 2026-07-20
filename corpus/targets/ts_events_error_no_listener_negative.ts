// SAFE: Register an 'error' listener before emitting errors
import { EventEmitter } from 'node:events';

class ConnectionPool extends EventEmitter {
  acquire(): void {
    this.emit('error', new Error('Pool exhausted'));
  }
}

function startServer(): void {
  const pool = new ConnectionPool();
  pool.on('error', (err) => {
    console.error('Connection pool error:', err.message);
  });
  pool.acquire();
}
