// SAFE alternative: Use AbortSignal for cleanup
import { EventEmitter } from 'node:events';

function pollUntil(emitter: EventEmitter, event: string, condition: (data: unknown) => boolean, signal?: AbortSignal): void {
  const handler = (data: unknown): void => {
    if (!condition(data)) return;
    emitter.removeListener(event, handler);
  };
  emitter.on(event, handler);
  signal?.addEventListener('abort', () => emitter.removeListener(event, handler));
}
