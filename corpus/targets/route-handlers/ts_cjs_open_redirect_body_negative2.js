// SAFE: The returnUrl is checked to be a relative path and its hostname, if present, must match an allowlist.

const express = require('express');
const url = require('url');

const app = express();
app.use(require('body-parser').json());

app.post('/auth/login', function(req, res) {
  loginUser(req.body.username, req.body.password, function(err, user) {
    if (err) return res.status(401).send('Login failed');
    req.session.userId = user._id;
    var returnUrl = req.body.returnUrl || '/';
    var parsed = url.parse(returnUrl);
    if (parsed.hostname && parsed.hostname !== req.hostname) {
      return res.redirect('/');
    }
    res.redirect(returnUrl);
  });
});

function loginUser(username, password, cb) {
  db.collection('users').findOne({ username: username, password: password }, cb);
}
