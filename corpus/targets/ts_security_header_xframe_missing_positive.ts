// [frensense]
// observation: X-Frame-Options or CSP frame-ancestors header is missing, allowing the page to be embedded in a frame on another origin.
// impact: Attackers can perform clickjacking: overlay invisible frames of the target page on top of decoy UI elements, tricking users into clicking buttons or submitting forms on the target site without their knowledge.
// improvement: Set X-Frame-Options: DENY (or SAMEORIGIN if framing is required) or use Content-Security-Policy: frame-ancestors 'self'.

import express from 'express';

const app = express();

// VULNERABLE: no X-Frame-Options header
app.get('/api/sensitive', (req, res) => {
  res.json({ data: sensitiveData });
});

app.get('/bank/transfer', (req, res) => {
  res.render('transfer-form');
});
