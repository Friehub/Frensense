// SAFE alternative: avoid using X-Forwarded-Host entirely; rely on the Host header and set Vary header
import express from 'express';

const app = express();

app.get('/redirect', (req, res) => {
  const host = req.headers.host as string;
  const redirectUrl = `https://${host}/callback?code=abc`;
  res.setHeader('Vary', 'Host');
  res.redirect(302, redirectUrl);
});

export function setupRedirectRoutes(server: express.Application): void {
  server.get('/auth/complete', (req, res) => {
    const host = req.headers.host as string;
    res.setHeader('Vary', 'Host');
    res.redirect(`https://${host}/welcome`);
  });
}
