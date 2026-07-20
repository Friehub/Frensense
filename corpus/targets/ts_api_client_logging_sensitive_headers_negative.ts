// SAFE: sensitive headers are redacted before logging

const SENSITIVE_HEADERS = new Set(['authorization', 'cookie', 'x-api-key', 'set-cookie']);

function redactHeaders(headers: Record<string, string>): Record<string, string> {
  const result: Record<string, string> = {};
  for (const [key, value] of Object.entries(headers)) {
    result[key] = SENSITIVE_HEADERS.has(key.toLowerCase()) ? '[REDACTED]' : value;
  }
  return result;
}

function logApiCall(url: string, headers: Record<string, string>, body: unknown) {
  console.log('API Request', { url, headers: redactHeaders(headers), body });
}

async function callApi() {
  logApiCall('https://api.example.com/data', {
    Authorization: 'Bearer sk-secret-token-12345',
    'Content-Type': 'application/json',
  }, { query: 'test' });
  return fetch('https://api.example.com/data', {
    headers: {
      Authorization: 'Bearer sk-secret-token-12345',
      'Content-Type': 'application/json',
    },
  });
}
