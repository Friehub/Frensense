// SAFE: Checks Origin/Referer header before processing POST requests

var express = require('express');

function checkOrigin(req, res, next) {
  var origin = req.headers['origin'];
  var referer = req.headers['referer'];
  if (!origin && !referer) {
    return res.status(403).json({ error: 'Origin or Referer header required' });
  }
  var source = origin || referer;
  var allowed = ['https://example.com', 'https://www.example.com'];
  var valid = allowed.some(function(a) {
    return source.indexOf(a) === 0;
  });
  if (!valid) {
    return res.status(403).json({ error: 'Cross-site request forbidden' });
  }
  next();
}

module.exports = function(app, db) {
  app.post('/settings/update', checkOrigin, function(req, res) {
    db.collection('settings').updateOne(
      { userId: req.session.userId },
      { $set: req.body },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Save failed' });
        res.json({ success: true });
      }
    );
  });

  app.post('/settings/delete', checkOrigin, function(req, res) {
    db.collection('settings').deleteOne(
      { userId: req.session.userId },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Delete failed' });
        res.json({ success: true });
      }
    );
  });
};
