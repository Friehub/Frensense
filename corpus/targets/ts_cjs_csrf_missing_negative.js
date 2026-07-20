var express = require('express');
var app = express();

function validateCsrfToken(req, res, next) {
    var token = req.headers['x-csrf-token'];
    if (!token || token !== req.session.csrfToken) {
        return res.status(403).json({ error: 'Invalid CSRF token' });
    }
    next();
}

app.post('/api/transfer', validateCsrfToken, function(req, res) {
    performTransfer(req.body);
    res.json({ status: 'ok' });
});

app.post('/api/update-profile', validateCsrfToken, function(req, res) {
    updateProfile(req.user.id, req.body);
    res.json({ status: 'ok' });
});
