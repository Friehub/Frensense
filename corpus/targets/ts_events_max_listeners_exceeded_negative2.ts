// SAFE alternative: Use a single listener dispatching to multiple handlers
import { EventEmitter } from 'node:events';

class Server extends EventEmitter {
  private handlers: Array<(data: unknown) => void> = [];

  addHandler(fn: (data: unknown) => void): void {
    this.handlers.push(fn);
    this.removeAllListeners('request');
    this.on('request', (data: unknown) => {
      for (const h of this.handlers) h(data);
    });
  }
}
