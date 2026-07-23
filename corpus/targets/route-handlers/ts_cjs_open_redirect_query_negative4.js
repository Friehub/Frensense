// SAFE: Redirect path is prefixed with '/' and sanitized to prevent open redirect
var express = require('express');

var app = express();

function sanitizeUrl(path) {
    return path.replace(/[^a-zA-Z0-9\-._~\/]/g, '').replace(/\/+/g, '/');
}

app.get('/auth/return', function(req, res) {
    var path = req.query.path;
    if (typeof path !== 'string' || !path.startsWith('/')) {
        return res.status(400).json({ error: 'Invalid path' });
    }
    res.redirect('/' + sanitizeUrl(path));
});

app.get('/logout', function(req, res) {
    var path = req.query.path;
    if (typeof path !== 'string' || !path.startsWith('/')) {
        return res.status(400).json({ error: 'Invalid path' });
    }
    res.redirect('/' + sanitizeUrl(path));
});
