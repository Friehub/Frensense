// SAFE: Uses jwt.verify with algorithms and audience validation
var jwt = require('jsonwebtoken');
var express = require('express');
var app = express();

var verifyOptions = {
    algorithms: ['HS256'],
    audience: 'api.frensense.io'
};

function authenticate(token) {
    return jwt.verify(token, process.env.JWT_SECRET, verifyOptions);
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
