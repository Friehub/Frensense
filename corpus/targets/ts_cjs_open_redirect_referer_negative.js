// SAFE: The Referer header is not used for redirect. A fixed redirect path is used instead.

const express = require('express');

const app = express();

app.post('/auth/logout', function(req, res) {
  req.session.destroy(function(err) {
    if (err) return res.status(500).send('Error');
    res.redirect('/');
  });
});
