// SAFE: Module-level state is replaced with a request-scoped context object

import { AsyncLocalStorage } from 'async_hooks';

const requestStorage = new AsyncLocalStorage<Map<string, AbortController>>();

export async function executeFetch(reqId: string) {
  const activeRequests = requestStorage.getStore() || new Map();
  const controller = new AbortController();
  activeRequests.set(reqId, controller);

  try {
    return await fetch("https://api.example.com", { signal: controller.signal });
  } finally {
    activeRequests.delete(reqId);
  }
}

export async function handler(request: Request) {
  const store = new Map<string, AbortController>();
  return requestStorage.run(store, () => {
    return executeFetch(request.headers.get('x-request-id') || 'default');
  });
}
