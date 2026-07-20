// [frensense]
// observation: The Express app uses res.redirect() with a URL taken directly from the query parameter without validation, allowing an attacker to redirect users to arbitrary external domains.
// impact: An attacker can craft a phishing link that redirects users from the legitimate site to a malicious site, stealing credentials or tokens.
// improvement: Validate the redirect URL against an allowlist of trusted domains, or only allow relative redirects.

import express from 'express';

const app = express();

app.get('/redirect', (req, res) => {
  const target = req.query.url as string;
  res.redirect(target);
});
