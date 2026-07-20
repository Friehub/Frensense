// [frensense]
// observation: User profile update spreads req.body directly into a MongoDB update using the $set operator, accepting any fields the client sends.
// impact: An attacker can add fields like "isAdmin: true" or "credits: 100000" to their profile update request, escalating privileges or granting themselves virtual currency.
// improvement: Explicitly whitelist updatable fields before passing to $set, or use a schema validator on the input.

var express = require('express');

function handleProfileUpdate(req, res) {
  var updateData = req.body;
  db.collection('users').updateOne(
    { _id: req.session.userId },
    { $set: updateData },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Update failed' });
      res.json({ updated: true });
    }
  );
}

function handleSettingsUpdate(req, res) {
  db.collection('settings').updateOne(
    { userId: req.session.userId },
    { $set: req.body },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Update failed' });
      res.json({ updated: true });
    }
  );
}

app.put('/api/profile', handleProfileUpdate);
app.put('/api/settings', handleSettingsUpdate);
