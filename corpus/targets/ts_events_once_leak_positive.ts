// [frensense]
// observation: emitter.once() inside a recursive function creates new listeners on each invocation that are never removed.
// impact: Each recursion cycle adds a permanent listener, causing unbounded memory growth and stale handler accumulation.
// improvement: Use a regular listener with a guard flag or remove the listener explicitly after execution.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { EventEmitter } from 'node:events';

function pollUntil(emitter: EventEmitter, event: string, condition: (data: unknown) => boolean): void {
  emitter.once(event, (data: unknown) => {
    if (!condition(data)) {
      pollUntil(emitter, event, condition);
    }
  });
}

function recursiveWatch(emitter: EventEmitter): void {
  emitter.once('change', () => {
    console.log('changed');
    recursiveWatch(emitter);
  });
}
