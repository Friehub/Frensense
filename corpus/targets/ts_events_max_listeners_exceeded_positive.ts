// [frensense]
// observation: More than 10 listeners registered on a single EventEmitter without increasing the max listener count.
// impact: Node.js emits a MaxListenersExceededWarning indicating a likely memory leak; excessive listeners degrade event dispatch performance.
// improvement: Either increase the limit with setMaxListeners(n) if intentional, or investigate and consolidate duplicate listeners.

import { EventEmitter } from 'node:events';

class Server extends EventEmitter {}

function registerHandlers(server: Server): void {
  for (let i = 0; i < 15; i++) {
    server.on('request', () => {
      console.log(`Handler ${i} called`);
    });
  }
}

function addManyListeners(emitter: EventEmitter): void {
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
  emitter.on('data', () => {});
}
