// SAFE: validates URL host against an allowlist before using it in a redirect
import express from 'express';

const ALLOWED_HOSTS = ['api.example.com', 'www.example.com', 'cdn.example.com'];

function validateRedirectUrl(toUrl: string): boolean {
  for (const allowed of ALLOWED_HOSTS) {
    if (toUrl.startsWith(`https://${allowed}/`)) {
      return true;
    }
  }
  return false;
}

const app = express();

app.get('/redirect', (req, res) => {
  const target = req.query.redirect as string;
  if (!target || !validateRedirectUrl(target)) {
    return res.status(400).send('Invalid redirect');
  }
  res.redirect(302, target);
});
