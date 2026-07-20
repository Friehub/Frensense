// [frensense]
// observation: DELETE endpoints accept cross-origin requests without CSRF token validation or Origin header checking, trusting cookie-based auth alone.
// impact: An attacker can forge a DELETE request from a malicious site using the victim's session cookie to delete resources without consent.
// improvement: Validate CSRF tokens, check Origin/Referer headers, or use SameSite=Strict cookies for state-changing DELETE requests.

var express = require('express');

function deleteAccount(req, res) {
  db.collection('accounts').deleteOne(
    { _id: req.session.accountId },
    function(err, result) {
      if (err) return res.status(500).json({ error: 'Delete failed' });
      res.json({ deleted: true });
    }
  );
}

function deleteFile(req, res) {
  fs.unlink('/uploads/' + req.params.filename, function(err) {
    if (err) return res.status(500).json({ error: 'Delete failed' });
    res.json({ deleted: true });
  });
}

app.delete('/api/account', deleteAccount);
app.delete('/api/files/:filename', deleteFile);
