// [frensense]
// observation: The application logs the entire request object or response body, which includes API keys or auth tokens from headers or query params.
// impact: Anyone with access to the log files (developers, SIEM systems, log management services) can extract valid API keys and use them for unauthorized access.
// improvement: Redact sensitive fields (authorization, x-api-key, password, token) before logging requests or responses.

export async function handleRequest(req: Request): Promise<Response> {
  console.log('Incoming request:', req.headers);
  console.log('Request body:', await req.text());
  const res = await fetch('https://api.example.com/data', {
    headers: { Authorization: `Bearer ${API_KEY}` }
  });
  console.log('Response:', res);
  return res;
}
