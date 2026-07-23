// SAFE: Redacts sensitive fields before logging
var express = require('express');
var app = express();

function redactSensitive(obj) {
    var sensitive = ['password', 'token', 'secret', 'authorization', 'api_key', 'session'];
    var redacted = {};
    for (var key in obj) {
        if (sensitive.indexOf(key.toLowerCase()) !== -1) {
            redacted[key] = '[REDACTED]';
        } else {
            redacted[key] = obj[key];
        }
    }
    return redacted;
}

function handleLogin(req, res) {
    console.log("Login body:", JSON.stringify(redactSensitive(req.body)));
    res.json({ success: true });
}

function processAuth(req, res) {
    console.log("Auth headers:", JSON.stringify(redactSensitive(req.headers)));
    res.json({ ok: true });
}
