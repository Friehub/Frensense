// [frensense]
// observation: The server uses req.body.returnUrl as the redirect target without any validation, allowing an attacker to supply an arbitrary external URL in the POST body.
// impact: An attacker can submit a form with a malicious returnUrl to redirect users to a phishing site after form submission, facilitating credential theft.
// improvement: Validate the returnUrl against an allowlist or only permit relative paths.
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium
// runtime_probe: redirect

const express = require('express');

const app = express();
app.use(require('body-parser').json());

app.post('/auth/login', function(req, res) {
  loginUser(req.body.username, req.body.password, function(err, user) {
    if (err) return res.status(401).send('Login failed');
    req.session.userId = user._id;
    res.redirect(req.body.returnUrl || '/');
  });
});

function loginUser(username, password, cb) {
  db.collection('users').findOne({ username: username, password: password }, cb);
}
