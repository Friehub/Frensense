// [frensense]
// observation: PUT, POST, and DELETE API endpoints accept requests without any CSRF token validation or origin/referer header check, trusting the browser's same-origin policy alone.
// impact: An attacker can use a cross-site form submission or fetch() from a malicious site to perform state-changing operations, since browsers automatically include cookies for the target origin.
// improvement: Validate CSRF tokens, check Origin/Referer headers against an allowlist, or use SameSite=Strict cookies for state-changing requests.

var express = require('express');

function setupRoutes(app, db) {
  function handleTransfer(req, res) {
    db.collection('accounts').updateOne(
      { userId: req.session.userId },
      { $inc: { balance: -req.body.amount } },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Transfer failed' });
        res.json({ success: true });
      }
    );
  }

  function handleUpdateProfile(req, res) {
    db.collection('users').updateOne(
      { _id: req.session.userId },
      { $set: req.body },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Update failed' });
        res.json({ success: true });
      }
    );
  }

  function handleDeletePost(req, res) {
    db.collection('posts').deleteOne(
      { _id: req.params.id, authorId: req.session.userId },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Delete failed' });
        res.json({ success: true });
      }
    );
  }

  app.post('/api/transfer', handleTransfer);
  app.put('/api/profile', handleUpdateProfile);
  app.delete('/api/posts/:id', handleDeletePost);
}

module.exports = setupRoutes;
