// SAFE: Origin header check for API CSRF protection

var express = require('express');

var allowedOrigins = ['https://myapp.com', 'https://www.myapp.com'];

function csrfApiCheck(req, res, next) {
  var origin = req.headers['origin'];
  var referer = req.headers['referer'];

  if (!origin && !referer) {
    return res.status(403).json({ error: 'Origin header required' });
  }

  var source = origin || referer;
  var isAllowed = allowedOrigins.some(function(allowed) {
    return source.indexOf(allowed) === 0;
  });

  if (!isAllowed) {
    return res.status(403).json({ error: 'Cross-site request forbidden' });
  }
  next();
}

module.exports = function(app, db) {
  app.post('/api/transfer', csrfApiCheck, function(req, res) {
    db.collection('accounts').updateOne(
      { userId: req.session.userId },
      { $inc: { balance: -req.body.amount } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Transfer failed' });
        res.json({ success: true });
      }
    );
  });

  app.put('/api/profile', csrfApiCheck, function(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: req.body },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });

  app.delete('/api/posts/:id', csrfApiCheck, function(req, res) {
    db.collection('posts').deleteOne(
      { _id: req.params.id, authorId: req.session.userId },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Delete failed' });
        res.json({ success: true });
      }
    );
  });
};
