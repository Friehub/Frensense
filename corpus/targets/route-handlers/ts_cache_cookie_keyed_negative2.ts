// SAFE alternative: include the Vary header to make caches key on cookies
import express from 'express';

const app = express();

app.get('/dashboard', (req, res) => {
  const theme = req.cookies.theme || 'light';
  const html = `<html><body class="${theme}"><h1>Welcome back</h1></body></html>`;
  res.setHeader('Content-Type', 'text/html');
  res.setHeader('Cache-Control', 'public, max-age=3600');
  res.setHeader('Vary', 'Cookie');
  res.send(html);
});

export function serveUserPage(req: express.Request, res: express.Response): void {
  const lang = req.cookies.lang || 'en';
  const banner = `<div data-lang="${lang}">Special offer</div>`;
  res.setHeader('Cache-Control', 'public, max-age=86400');
  res.setHeader('Vary', 'Cookie');
  res.send(banner);
}
