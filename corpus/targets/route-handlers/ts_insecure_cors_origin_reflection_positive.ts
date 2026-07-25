// [frensense]
// observation: The Origin header from the request is directly reflected in the Access-Control-Allow-Origin response header.
// impact: Any malicious website can perform credentialed cross-origin requests to the API, bypassing CORS protections completely.
// improvement: Validate the Origin against a strict whitelist array before echoing it, or use a hardcoded domain.
// cwe: CWE-942
// cvss: 8.8
// owasp: A05:2021
// severity: High

app.use((req, res, next) => {
  // VULNERABLE: Origin header blindly trusted
  const origin = req.headers.origin;
  if (origin) {
    res.setHeader('Access-Control-Allow-Origin', origin);
    res.setHeader('Access-Control-Allow-Credentials', 'true');
  }
  next();
});

export async function middleware(request: NextRequest) {
  // VULNERABLE: reflecting origin in Next.js
  const origin = request.headers.get('origin');
  const response = NextResponse.next();
  if (origin) {
    response.headers.set('Access-Control-Allow-Origin', origin);
  }
  return response;
}
