// [frensense]
// observation: Form submission endpoints accept POST requests but never validate a CSRF token against the session, making them vulnerable to cross-site request forgery.
// impact: An attacker can craft a malicious HTML form that submits to this endpoint from another site, performing actions like changing email, password, or transferring funds without the victim's consent.
// improvement: Generate a CSRF token, embed it in forms, and validate it on the server against the session-stored token before processing the request.
// cwe: CWE-352
// cvss: 8.8
// owasp: A01:2021
// severity: High

var express = require('express');
var app = express();

function handlePasswordReset(req, res) {
  var newPass = req.body.newPass;
  var confirmPass = req.body.confirmPass;

  if (newPass !== confirmPass) {
    return res.status(400).json({ error: 'Passwords do not match' });
  }

  db.collection('accounts').updateOne(
    { email: req.session.email },
    { $set: { password: newPass } },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Reset failed' });
      res.json({ message: 'Password updated successfully' });
    }
  );
}

function addPaymentMethod(req, res) {
  var cardToken = req.body.cardToken;

  db.collection('payment_methods').insertOne(
    { userId: req.session.userId, token: cardToken },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Could not add card' });
      res.json({ id: result.insertedId });
    }
  );
}

app.post('/account/reset-password', handlePasswordReset);
app.post('/account/payment-method', addPaymentMethod);
