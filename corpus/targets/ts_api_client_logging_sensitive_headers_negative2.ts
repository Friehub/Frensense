// SAFE: a wrapper logger ensures sensitive headers are never logged

const SENSITIVE_HEADERS = new Set(['authorization', 'cookie', 'x-api-key']);

function sanitizeLogEntry(entry: Record<string, unknown>): Record<string, unknown> {
  if (entry.headers && typeof entry.headers === 'object') {
    const sanitized: Record<string, string> = {};
    for (const [k, v] of Object.entries(entry.headers)) {
      sanitized[k] = SENSITIVE_HEADERS.has(k.toLowerCase()) ? '[REDACTED]' : String(v);
    }
    return { ...entry, headers: sanitized };
  }
  return entry;
}

const apiLogger = {
  info: (msg: string, data: Record<string, unknown>) => {
    console.log(msg, sanitizeLogEntry(data));
  },
};

async function callApi() {
  const headers = {
    Authorization: 'Bearer sk-secret-token-12345',
    'Content-Type': 'application/json',
  };
  apiLogger.info('API Request', { url: 'https://api.example.com/data', headers, body: { query: 'test' } });
  return fetch('https://api.example.com/data', { headers });
}
