// SAFE: Uses helmet's cookie security defaults via a global middleware
var helmet = require('helmet');
var express = require('express');
var app = express();

app.use(helmet());

function handler(req, res) {
    res.cookie("session", "value", { httpOnly: true, secure: true, sameSite: "strict" });
}

app.get('/login', handler);
