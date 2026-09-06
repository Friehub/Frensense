// SAFE alternative: Manual destructuring of allowed fields

var express = require('express');

module.exports = function(app, db) {
  app.post('/user/update-profile', function(req, res) {
    var updates = {};
    if (req.body.displayName) updates.displayName = req.body.displayName;
    if (req.body.bio) updates.bio = req.body.bio;
    if (req.body.email) updates.email = req.body.email;
    if (req.body.phoneNumber) updates.phoneNumber = req.body.phoneNumber;

    db.collection('users').update(
      { _id: req.session.userId },
      { $set: updates },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  });
};
