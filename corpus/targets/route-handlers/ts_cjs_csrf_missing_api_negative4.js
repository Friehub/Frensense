// SAFE: API using SameSite=Strict cookies with CSRF double-submit pattern

var express = require('express');
var crypto = require('crypto');

function doubleSubmitCsrf(req, res, next) {
  var cookieToken = req.cookies['csrf'];
  var headerToken = req.headers['x-csrf-token'];
  if (!cookieToken || !headerToken) {
    return res.status(403).json({ error: 'CSRF tokens missing' });
  }
  if (cookieToken !== headerToken) {
    return res.status(403).json({ error: 'CSRF token mismatch' });
  }
  next();
}

module.exports = function(app, db) {
  app.get('/api/csrf', function(req, res) {
    var token = crypto.randomBytes(32).toString('hex');
    res.cookie('csrf', token, { httpOnly: false, sameSite: 'strict', secure: true });
    res.json({ token: token });
  });

  app.put('/api/profile', doubleSubmitCsrf, function(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: { displayName: req.body.displayName } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });

  app.post('/api/transfer', doubleSubmitCsrf, function(req, res) {
    db.collection('accounts').updateOne(
      { userId: req.session.userId },
      { $inc: { balance: -req.body.amount } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Transfer failed' });
        res.json({ success: true, newBalance: result.balance });
      }
    );
  });
};
