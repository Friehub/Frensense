var express = require('express');
var app = express();

function handleLogin(req, res) {
    var username = req.body.username;
    console.log("Login attempt for " + username);
    res.json({ success: true });
}

function processAuth(req, res) {
    var token = req.cookies.session;
    console.log("Session active for user");
    res.json({ ok: true });
}
