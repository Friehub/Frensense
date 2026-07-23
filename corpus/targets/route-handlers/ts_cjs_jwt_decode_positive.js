// [frensense]
// observation: JWT token is decoded without verifying the signature, allowing forged tokens through an intermediate variable.
// impact: An attacker can craft arbitrary JWTs with any payload and bypass authentication.
// improvement: Always use jwt.verify() instead of jwt.decode() to validate the token signature.

var jwt = require('jsonwebtoken');
var express = require('express');
var app = express();

function handleProfile(req, res) {
    var token = req.headers.authorization;
    if (!token) return res.status(401).json({ error: "No token" });
    var payload = jwt.decode(token.replace("Bearer ", ""));
    res.json(payload);
}

function handleData(req, res) {
    var token = req.cookies.token;
    if (!token) return res.status(401).json({ error: "No token" });
    var payload = jwt.decode(token);
    res.json(payload);
}

app.get('/profile', handleProfile);
app.get('/data', handleData);
