// SAFE: Uses jwt.verify with audience and issuer checks
var jwt = require('jsonwebtoken');
var express = require('express');
var app = express();

function verifyToken(token) {
    return jwt.verify(token, process.env.JWT_SECRET, {
        audience: process.env.JWT_AUDIENCE,
        issuer: process.env.JWT_ISSUER
    });
}

app.get('/profile', function(req, res) {
    var auth = req.headers.authorization;
    if (!auth) return res.status(401).json({ error: "No token" });
    var token = auth.replace("Bearer ", "");
    try {
        var payload = verifyToken(token);
        res.json(payload);
    } catch (e) {
        res.status(401).json({ error: "Invalid token" });
    }
});

app.get('/data', function(req, res) {
    var token = req.cookies.token;
    if (!token) return res.status(401).json({ error: "No token" });
    try {
        var payload = verifyToken(token);
        res.json(payload);
    } catch (e) {
        res.status(401).json({ error: "Invalid token" });
    }
});
