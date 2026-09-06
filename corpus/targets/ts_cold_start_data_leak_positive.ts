// [frensense]
// observation: In a serverless environment, data from a previous request stored at module scope is visible to a new request during cold-start reuse.
// impact: User A's private data may be returned to User B when the cloud provider reuses the same execution environment.
// improvement: Never store per-request data in module-level variables. Use external storage like KV or databases.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

const requestCache = new Map<string, any>();

export async function handler(request: Request): Promise<Response> {
  const userId = request.headers.get('X-User-Id') || 'anonymous';
  const url = new URL(request.url);

  if (requestCache.has(url.pathname)) {
    return new Response(JSON.stringify(requestCache.get(url.pathname)));
  }

  const data = await fetchData(userId, url.pathname);
  requestCache.set(url.pathname, data);
  return new Response(JSON.stringify(data));
}

async function fetchData(userId: string, path: string): Promise<any> {
  return { userId, path, timestamp: Date.now() };
}
