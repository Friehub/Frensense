var express = require('express');
var app = express();

var cookieConfig = {
    httpOnly: true,
    secure: true,
    sameSite: 'strict'
};

function handler(req, res) {
    var token = "some-session-value";
    res.cookie("session", token, cookieConfig);
    res.json({ ok: true });
}

app.get('/login', handler);
