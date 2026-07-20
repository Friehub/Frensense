// SAFE: Sensitive headers are redacted before logging
const SENSITIVE_HEADERS = ['authorization', 'x-api-key', 'cookie', 'set-cookie'];

function sanitizeHeaders(headers: Headers): Record<string, string> {
  const sanitized: Record<string, string> = {};
  headers.forEach((value, key) => {
    sanitized[key] = SENSITIVE_HEADERS.includes(key.toLowerCase()) ? '[REDACTED]' : value;
  });
  return sanitized;
}

export async function handleRequest(req: Request): Promise<Response> {
  console.log('Incoming request headers:', sanitizeHeaders(req.headers));
  const body = await req.text();
  const res = await fetch('https://api.example.com/data', {
    headers: { Authorization: `Bearer ${process.env.API_KEY}` }
  });
  return res;
}
