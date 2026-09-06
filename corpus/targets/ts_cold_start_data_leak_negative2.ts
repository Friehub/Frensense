// SAFE: Request-scoped data is stored using AsyncLocalStorage, never at module level

import { AsyncLocalStorage } from 'async_hooks';

const requestStorage = new AsyncLocalStorage<Map<string, any>>();

export async function handler(request: Request): Promise<Response> {
  const store = new Map();
  return requestStorage.run(store, async () => {
    const userId = request.headers.get('X-User-Id') || 'anonymous';
    const url = new URL(request.url);

    const cached = store.get(url.pathname);
    if (cached) {
      return new Response(JSON.stringify(cached));
    }

    const data = await fetchData(userId, url.pathname);
    store.set(url.pathname, data);
    return new Response(JSON.stringify(data));
  });
}

async function fetchData(userId: string, path: string): Promise<any> {
  return { userId, path, timestamp: Date.now() };
}
