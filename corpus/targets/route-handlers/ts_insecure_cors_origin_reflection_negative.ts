// SAFE: Origin validated against a whitelist
const ALLOWED_ORIGINS = ['https://friehub.com', 'https://taas.friehub.com'];

app.use((req, res, next) => {
  const origin = req.headers.origin;
  // SAFE: Strict equality check against an allowed list
  if (origin && ALLOWED_ORIGINS.includes(origin)) {
    res.setHeader('Access-Control-Allow-Origin', origin);
    res.setHeader('Access-Control-Allow-Credentials', 'true');
  }
  next();
});

export async function middleware(request: NextRequest) {
  const origin = request.headers.get('origin');
  const response = NextResponse.next();
  // SAFE: regex testing against a trusted internal pattern
  if (origin && /^https:\/\/([a-z0-9-]+\.)?friehub\.com$/.test(origin)) {
    response.headers.set('Access-Control-Allow-Origin', origin);
  }
  return response;
}
