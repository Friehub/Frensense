// SAFE: The Referer header is parsed and only the path portion is used for the redirect.

const express = require('express');
const url = require('url');

const app = express();

app.post('/auth/logout', function(req, res) {
  req.session.destroy(function(err) {
    if (err) return res.status(500).send('Error');
    var referer = req.get('Referer');
    if (referer) {
      var parsed = url.parse(referer);
      if (parsed.hostname === req.hostname) {
        return res.redirect(parsed.path || '/');
      }
    }
    res.redirect('/');
  });
});
