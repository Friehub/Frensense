// [frensense]
// observation: No Referrer-Policy header is set, allowing the full URL (including query parameters with tokens or session IDs) to be sent in the Referer header to external sites.
// impact: When users click external links from the application, the full URL (including OAuth tokens, password reset tokens, or session IDs in query params) is leaked to the third-party domain via the Referer header.
// improvement: Set Referrer-Policy to no-referrer, same-origin, or strict-origin-when-cross-origin.

import express from 'express';

const app = express();

// VULNERABLE: no Referrer-Policy
app.get('/dashboard', (req, res) => {
  res.send('<a href="https://external.example.com">Click me</a>');
});

app.get('/payment', (req, res) => {
  res.send(`<form action="/confirm" method="POST">
    <input type="hidden" name="token" value="${req.query.token}" />
    <button type="submit">Pay</button>
  </form>`);
});
