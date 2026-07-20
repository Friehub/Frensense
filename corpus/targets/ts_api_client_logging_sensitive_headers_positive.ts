// [frensense]
// observation: API client logs include sensitive request or response headers such as Authorization, Cookie, or X-Api-Key.
// impact: Sensitive credentials are written to application logs, which may be stored indefinitely, shipped to log aggregation services, and viewed by operators who should not have access to secrets. This violates security compliance standards like SOC2 and PCI-DSS.
// improvement: Filter or redact sensitive headers before logging request/response data.

function logApiCall(url: string, headers: Record<string, string>, body: unknown) {
  console.log('API Request', { url, headers, body });
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
