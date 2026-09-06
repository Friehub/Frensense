// [frensense]
// observation: Form submission endpoints accept POST requests but never validate a CSRF token against the session, making them vulnerable to cross-site request forgery.
// impact: An attacker can craft a malicious HTML form that submits to this endpoint from another site, performing actions like changing email, password, or transferring funds without the victim's consent.
// improvement: Generate a CSRF token, embed it in forms, and validate it on the server against the session-stored token before processing the request.

var express = require('express');
var router = express.Router();

function removeAccount(req, res) {
  var confirm = req.body.confirm;

  if (confirm === 'DELETE') {
    db.collection('users').deleteOne(
      { _id: req.session.userId },
      function(err, result) {
        if (err) return res.status(500).json({ error: 'Removal failed' });
        req.session.destroy();
        res.json({ removed: true });
      }
    );
  } else {
    res.status(400).json({ error: 'Confirmation text mismatch' });
  }
}

function updateProfile(req, res) {
  var displayName = req.body.displayName;
  var bio = req.body.bio;

  db.collection('users').updateOne(
    { _id: req.session.userId },
    { $set: { displayName: displayName, bio: bio } },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Update failed' });
      res.json({ success: true });
    }
  );
}

router.post('/account/delete', removeAccount);
router.post('/account/profile', updateProfile);

module.exports = router;
