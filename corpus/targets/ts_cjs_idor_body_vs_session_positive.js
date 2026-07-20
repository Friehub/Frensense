// [frensense]
// observation: The userId is taken from req.body instead of the authenticated session, allowing an attacker to arbitrarily change which user's data is operated on by modifying the request body.
// impact: An attacker can submit a crafted POST request with any userId value to modify other users' accounts, perform unauthorized transfers, or escalate privileges.
// improvement: Derive the userId from the authenticated session (req.session.userId) and ignore user-supplied identifiers.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/profile/update', function(req, res) {
  var update = { $set: { email: req.body.email, name: req.body.name } };
  db.collection('profiles').updateOne({ userId: req.body.userId }, update, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json({ success: true });
  });
});
