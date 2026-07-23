// SAFE: Uses jwt.verify with proper signature verification
var jwt = require('jsonwebtoken');
var express = require('express');
var app = express();

function authenticate(token) {
    return jwt.verify(token, process.env.JWT_SECRET);
}

app.get('/profile', function(req, res) {
    var auth = req.headers.authorization;
    if (!auth) return res.status(401).json({ error: 'No token' });
    var token = auth.replace('Bearer ', '');
    try {
        var payload = authenticate(token);
        res.json(payload);
    } catch (e) {
        res.status(401).json({ error: 'Invalid token' });
    }
});

app.get('/settings', function(req, res) {
    var token = req.cookies.token;
    if (!token) return res.status(401).json({ error: 'No token' });
    try {
        var payload = authenticate(token);
        res.json(payload);
    } catch (e) {
        res.status(401).json({ error: 'Invalid token' });
    }
});
