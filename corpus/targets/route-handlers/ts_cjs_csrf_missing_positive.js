// [frensense]
// observation: CSRF token is sent in the request but the server never validates it against the session.
// impact: Any cross-site request forgery attack succeeds because the token check is missing. The token's presence is meaningless if it isn't verified.
// improvement: Always compare the submitted CSRF token against the one stored in the user's session on the server. If they don't match, reject the request.

var express = require('express');
var app = express();

function validateCsrfToken(req, res, next) {
    var token = req.headers['x-csrf-token'];
    // VULNERABLE: token sent but never validated
    // Should be: if (token !== req.session.csrfToken) return res.status(403).json(...)
    next();
}

app.post('/api/transfer', validateCsrfToken, function(req, res) {
    performTransfer(req.body);
    res.json({ status: 'ok' });
});

app.post('/api/update-profile', function(req, res) {
    // VULNERABLE: no CSRF protection at all
    updateProfile(req.user.id, req.body);
    res.json({ status: 'ok' });
});
