// SAFE: res.cookie with httpOnly, secure, sameSite all set

var express = require('express');
var app = express();

function login(req, res) {
  var token = 'sess_' + Math.random().toString(36).slice(2);
  res.cookie('session', token, {
    httpOnly: true,
    secure: true,
    sameSite: 'strict',
    maxAge: 3600000
  });
  res.json({ ok: true, loggedIn: true });
}

function logout(req, res) {
  res.cookie('session', '', {
    httpOnly: true,
    secure: true,
    sameSite: 'strict',
    maxAge: 0
  });
  res.json({ ok: true, loggedOut: true });
}

app.post('/login', login);
app.post('/logout', logout);
