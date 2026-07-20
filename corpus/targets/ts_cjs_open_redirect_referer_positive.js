// [frensense]
// observation: The server uses req.get('Referer') as the redirect target without validation, trusting the Referer header which can be spoofed by the attacker.
// impact: An attacker can craft a request with a malicious Referer header pointing to their phishing site, and the server will redirect the user there after processing, enabling phishing attacks.
// improvement: Do not rely on the Referer header for redirect destinations. Always validate against an allowlist or use a session-stored return URL.

const express = require('express');

const app = express();

app.post('/auth/logout', function(req, res) {
  req.session.destroy(function(err) {
    if (err) return res.status(500).send('Error');
    res.redirect(req.get('Referer') || '/');
  });
});
