// SAFE alternative: Use async context to catch errors
import { EventEmitter } from 'node:events';

class Database extends EventEmitter {
  async connect(): Promise<void> {
    try {
      this.emit('connecting');
    } catch (err) {
      this.emit('error', err);
    }
  }
}
