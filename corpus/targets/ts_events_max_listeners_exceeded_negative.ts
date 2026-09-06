// SAFE: Increase max listeners or consolidate handlers
import { EventEmitter } from 'node:events';

class Server extends EventEmitter {
  constructor() {
    super();
    this.setMaxListeners(20);
  }
}

function registerHandlers(server: Server): void {
  server.on('request', (req) => {
    console.log(`Handling ${req}`);
  });
}

function addManyListeners(emitter: EventEmitter): void {
  emitter.setMaxListeners(20);
  for (let i = 0; i < 15; i++) {
    emitter.on('data', () => {});
  }
}
