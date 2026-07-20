// SAFE: The userId is sourced from req.session.userId, preventing body manipulation for privilege escalation.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/profile/update', function(req, res) {
  var update = { $set: { email: req.body.email, name: req.body.name } };
  db.collection('profiles').updateOne({ userId: req.session.userId }, update, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json({ success: true });
  });
});
