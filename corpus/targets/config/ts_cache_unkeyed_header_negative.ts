// SAFE: validate X-Forwarded-Host against a whitelist before using it
import express from 'express';

const ALLOWED_HOSTS = new Set(['api.example.com', 'www.example.com']);

function getSafeHost(req: express.Request): string {
  const forwarded = req.headers['x-forwarded-host'] as string | undefined;
  if (forwarded && ALLOWED_HOSTS.has(forwarded)) return forwarded;
  return req.headers.host as string;
}

const app = express();

app.get('/redirect', (req, res) => {
  const host = getSafeHost(req);
  const redirectUrl = `https://${host}/callback?code=abc`;
  res.redirect(302, redirectUrl);
});

export function setupRedirectRoutes(server: express.Application): void {
  server.get('/auth/complete', (req, res) => {
    const target = getSafeHost(req);
    res.redirect(`https://${target}/welcome`);
  });
}
