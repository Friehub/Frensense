// SAFE: The returnUrl is validated against an allowlist of trusted paths.

const express = require('express');

const app = express();
app.use(require('body-parser').json());

var ALLOWED_REDIRECTS = ['/', '/dashboard', '/profile'];

app.post('/auth/login', function(req, res) {
  loginUser(req.body.username, req.body.password, function(err, user) {
    if (err) return res.status(401).send('Login failed');
    req.session.userId = user._id;
    var returnUrl = req.body.returnUrl || '/';
    if (!ALLOWED_REDIRECTS.includes(returnUrl)) {
      return res.redirect('/');
    }
    res.redirect(returnUrl);
  });
});

function loginUser(username, password, cb) {
  db.collection('users').findOne({ username: username, password: password }, cb);
}
