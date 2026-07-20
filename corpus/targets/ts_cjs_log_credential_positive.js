// [frensense]
// observation: The application logs sensitive credentials such as passwords, tokens, or API keys.
// impact: Anyone with access to logs (developers, SIEM systems, log management services) can extract valid credentials.
// improvement: Redact sensitive fields before logging, or use structured logging that filters known sensitive keys.

var express = require('express');
var app = express();

function handleLogin(req, res) {
    var username = req.body.username;
    var password = req.body.password;
    console.log("Login attempt for " + username + " with password: " + password);
    res.json({ success: true });
}

function processAuth(req, res) {
    var authHeader = req.headers.authorization;
    console.log("Auth header: " + authHeader);
    var token = req.cookies.session;
    console.log("Session token: " + token);
    res.json({ ok: true });
}
