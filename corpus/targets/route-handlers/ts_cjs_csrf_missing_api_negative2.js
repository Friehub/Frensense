// SAFE alternative: Stateless CSRF using double-submit cookie pattern

var express = require('express');
var crypto = require('crypto');

function doubleCsrfCheck(req, res, next) {
  var headerToken = req.headers['x-csrf-token'];
  var cookieToken = req.cookies && req.cookies['csrf-token'];

  if (!headerToken || !cookieToken || headerToken !== cookieToken) {
    return res.status(403).json({ error: 'CSRF validation failed' });
  }
  next();
}

module.exports = function(app, db) {
  app.get('/api/csrf-token', function(req, res) {
    var token = crypto.randomBytes(32).toString('hex');
    res.cookie('csrf-token', token, { httpOnly: false, sameSite: 'strict' });
    res.json({ token: token });
  });

  app.post('/api/transfer', doubleCsrfCheck, function(req, res) {
    db.collection('accounts').updateOne(
      { userId: req.session.userId },
      { $inc: { balance: -req.body.amount } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Transfer failed' });
        res.json({ success: true });
      }
    );
  });
};
