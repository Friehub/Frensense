// SAFE alternative: Destructure only known fields from req.body

var express = require('express');

module.exports = function(app, db) {
  app.put('/api/profile', function(req, res) {
    var displayName = req.body.displayName;
    var email = req.body.email;
    var bio = req.body.bio;

    var updates = {};
    if (displayName !== undefined) updates.displayName = displayName;
    if (email !== undefined) updates.email = email;
    if (bio !== undefined) updates.bio = bio;

    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: updates },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });
};
