// SAFE: Remove listener explicitly after use instead of re-registering
import { EventEmitter } from 'node:events';

function pollUntil(emitter: EventEmitter, event: string, condition: (data: unknown) => boolean): void {
  const handler = (data: unknown): void => {
    emitter.removeListener(event, handler);
    if (!condition(data)) {
      emitter.once(event, handler);
    }
  };
  emitter.once(event, handler);
}

function watchWithCleanup(emitter: EventEmitter): void {
  const handler = (): void => {
    emitter.removeListener('change', handler);
    console.log('changed');
  };
  emitter.on('change', handler);
}
