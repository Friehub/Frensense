// SAFE: Redirect URL validated against a host allowlist
var express = require('express');
var url = require('url');

var app = express();
var ALLOWED_HOSTS = ['example.com', 'app.example.com', 'help.example.com'];

app.get('/auth/return', function(req, res) {
    var target = req.query.returnTo;
    var parsed = url.parse(target);
    if (!ALLOWED_HOSTS.includes(parsed.hostname)) {
        return res.status(400).json({ error: 'Redirect not allowed' });
    }
    res.redirect(target);
});

app.get('/logout', function(req, res) {
    var target = req.query.next;
    var parsed = url.parse(target);
    if (!ALLOWED_HOSTS.includes(parsed.hostname)) {
        return res.status(400).json({ error: 'Redirect not allowed' });
    }
    res.redirect(target);
});
