// SAFE: The operation verifies ownership by matching both the body-supplied userId and the session userId.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/profile/update', function(req, res) {
  if (req.body.userId !== req.session.userId) {
    return res.status(403).json({ error: 'Forbidden' });
  }
  var update = { $set: { email: req.body.email, name: req.body.name } };
  db.collection('profiles').updateOne({ userId: req.session.userId }, update, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json({ success: true });
  });
});
